use std::fs::File;
use std::io::{prelude::*, BufReader};
extern crate bmp;
use bmp::{Image,Pixel};

#[derive(Debug)]
enum PanelMode {
    COLOR,
    MOVE,
}

#[derive(Debug)]
enum PanelFacing {
    UP,
    LEFT,
    DOWN,
    RIGHT
}

#[derive(Debug)]
struct Panel {
    data: Vec<u8>,
    width: i64,
    height: i64,
    loc: (i64,i64),
    min: (i64,i64),
    max: (i64,i64),
    facing: PanelFacing,
    mode: PanelMode,
}

impl Panel {
    fn new() -> Self {
        let mut ret = Panel{
            width: 5000,
            height: 5000,
            loc: (2500,2500),
            min: (std::i64::MAX,std::i64::MAX),
            max: (0,0),
            data: vec![255; 5000 * 5000],
            facing: PanelFacing::UP,
            mode: PanelMode::COLOR,
            // 255 == black, 0 == black painted, 1 == white painted
        };
        ret.data[2500 + 2500*5000] = 1;
        ret
    }

    fn get(&self) -> u8 {
        match self.data[(self.loc.0+self.loc.1*self.width) as usize] {
            0 | 255 => 0,
            1 => 1,
            _ => 255,
        }
    }

    fn move_robot(&mut self, d: (i64,i64)) {
        match (self.loc.0,d.0) {
            (0,-1) => {
                // self.width *= 2;
                // self.height *= 2;
                // self.data.resize(self.width * self.height,255);
            },
            (n,1) if n == (self.width-1) => {

            },
            (_,_) => ()
        };
        match (self.loc.1,d.1) {
            (0,-1) => {

            },
            (n,1) if n == (self.height-1) => {

            },
            (_,_) => ()
        }
        self.loc.0 += d.0;
        self.loc.1 += d.1;
        if self.loc.0 < self.min.0 {
            self.min.0 = self.loc.0;
        }
        if self.loc.1 < self.min.1 {
            self.min.1 = self.loc.1;
        }
        if self.loc.0 > self.max.0 {
            self.max.0 = self.loc.0;
        }
        if self.loc.1 > self.max.1 {
            self.max.1 = self.loc.1;
        }
    }

    fn input(&mut self, val: i64) {
        match self.mode {
            PanelMode::COLOR => {
                self.data[(self.loc.0+self.loc.1*self.width) as usize] = val as u8;
                self.mode = PanelMode::MOVE;
            },
            PanelMode::MOVE => {
                match (val,&self.facing) {
                    (0,PanelFacing::UP) => {
                        self.facing = PanelFacing::LEFT;
                        self.move_robot((-1,0));
                    },
                    (0,PanelFacing::RIGHT) => {
                        self.facing = PanelFacing::UP;
                        self.move_robot((0,1));
                    },
                    (0,PanelFacing::DOWN) => {
                        self.facing = PanelFacing::RIGHT;
                        self.move_robot((1,0));
                    },
                    (0,PanelFacing::LEFT) => {
                        self.facing = PanelFacing::DOWN;
                        self.move_robot((0,-1));
                    },
                    (1,PanelFacing::UP) => {
                        self.facing = PanelFacing::RIGHT;
                        self.move_robot((1,0));
                    },
                    (1,PanelFacing::RIGHT) => {
                        self.facing = PanelFacing::DOWN;
                        self.move_robot((0,-1));
                    },
                    (1,PanelFacing::DOWN) => {
                        self.facing = PanelFacing::LEFT;
                        self.move_robot((-1,0));
                    },
                    (1,PanelFacing::LEFT) => {
                        self.facing = PanelFacing::UP;
                        self.move_robot((0,1));
                    },
                    (_,_) => ()
                }
                self.mode = PanelMode::COLOR;
            }
        }
    }
}

#[derive(Debug)]
enum Op {
    INVALID=0,
    ADD=1,
    MULTIPLY=2,
    STORE=3,
    PRINT=4,
    JT=5,
    JF=6,
    LT=7,
    EQ=8,
    REL=9,
    HALT=99,
}

#[derive(Debug)]
enum Mode {
    POSITION = 0,
    IMMEDIATE = 1,
    RELATIVE = 2,
}

fn get_parameters(op: i32) -> (Op, Mode, Mode, Mode ) {
    (match op % 100 {
        1 => Op::ADD,
        2 => Op::MULTIPLY,
        3 => Op::STORE,
        4 => Op::PRINT,
        5 => Op::JT,
        6 => Op::JF,
        7 => Op::LT,
        8 => Op::EQ,
        9 => Op::REL,
        99 => Op::HALT,
        _ => Op::INVALID,
    },
    match (op / 100) % 10 {
        0 => Mode::POSITION,
        1 => Mode::IMMEDIATE,
        2 => Mode::RELATIVE,
        _ => Mode::POSITION,
    },
    match (op / 1000) % 10 {
        0 => Mode::POSITION,
        1 => Mode::IMMEDIATE,
        2 => Mode::RELATIVE,
        _ => Mode::POSITION,
    },
    match (op / 10000) % 10 {
        0 => Mode::POSITION,
        1 => Mode::IMMEDIATE,
        2 => Mode::RELATIVE,
        _ => Mode::POSITION,
    })
}

type ProgramType = Vec<i64>;
type PcType = usize;

fn ensure_location(program: &mut ProgramType, location: usize) {
    let mut ptr = program.len();
    while ptr <= location {
        ptr += 1;
        program.push(0);
    }
}

fn write_location(program: &mut ProgramType, location: usize, value: i64) {
    ensure_location(program, location);
    program[location] = value;
}

fn get_operand(mode: Mode, program: &mut ProgramType, pc: usize, relative: i64) -> i64 {
    match mode {
        Mode::POSITION => {
            ensure_location(program, program[pc] as usize);
            program[program[pc] as usize]
        },
        Mode::IMMEDIATE => {
            ensure_location(program, pc as usize);
            program[pc]
        },
        Mode::RELATIVE => {
            ensure_location(program, (relative + program[pc]) as usize);
            program[(relative + program[pc]) as usize]
        },
    }
}

fn get_operand_for_write(mode: Mode, program: &mut ProgramType, pc: usize, relative: i64) -> i64 {
    match mode {
        Mode::RELATIVE => {
            ensure_location(program, (relative + program[pc]) as usize);
            relative + program[pc]
        },
        _ => {
            ensure_location(program, program[pc] as usize);
            program[pc]
        },
    }
}

fn run(program: &mut ProgramType) -> Panel {
    let mut relative : i64 = 0;
    let mut pc: PcType = 0;
    let mut panel = Panel::new();

    while pc < program.len() {

        let (op,mode1,mode2,mode3) = get_parameters(program[pc] as i32);
        // println!("Program: {:?}", &program[pc..]);
        // println!("{} Handling {} -> {:?},{:?},{:?},{:?}", pc, program[pc],op,mode1,mode2,mode3);

        match op {
            Op::ADD => {
                let operand1 = get_operand(mode1, program, pc+1 as usize, relative);
                let operand2 = get_operand(mode2, program, pc+2 as usize, relative);
                let output = get_operand_for_write(mode3, program, pc+3 as usize, relative);
                write_location(program,output as usize,operand1 + operand2);
                pc += 4;
            },
            Op::MULTIPLY => {
                let operand1 = get_operand(mode1, program, pc+1 as usize, relative);
                let operand2 = get_operand(mode2, program, pc+2 as usize, relative);
                let output = get_operand_for_write(mode3, program, pc+3 as usize, relative);
                write_location(program,output as usize,operand1 * operand2);
                pc += 4;
            },
            Op::STORE => {
                let output_idx = get_operand_for_write(mode1, program, pc+1 as usize, relative);
                write_location(program,output_idx as usize, panel.get() as i64);
                pc += 2;
            },
            Op::PRINT => {
                let operand1 = get_operand(mode1, program, pc+1 as usize, relative);
//                println!("{}", operand1);
                panel.input(operand1);
                pc += 2;
            },
            Op::JT => {
                let operand1 = get_operand(mode1, program, pc+1 as usize, relative);
                let operand2 = get_operand(mode2, program, pc+2 as usize, relative);
                if operand1 != 0 {
                    pc = operand2 as usize;
                } else {
                    pc += 3;
                }
            },
            Op::JF => {
                let operand1 = get_operand(mode1, program, pc+1 as usize, relative);
                let operand2 = get_operand(mode2, program, pc+2 as usize, relative);
                if operand1 == 0 {
                    pc = operand2 as usize;
                } else {
                    pc += 3;
                }
            },
            Op::LT => {
                let operand1 = get_operand(mode1, program, pc+1 as usize, relative);
                let operand2 = get_operand(mode2, program, pc+2 as usize, relative);
                let output = get_operand_for_write(mode3, program, pc+3 as usize, relative) as usize;
                ensure_location(program, output as usize);
                program[output] = match operand1 < operand2 {
                    true => 1,
                    false => 0,
                };
                pc += 4;
            },
            Op::EQ => {
                let operand1 = get_operand(mode1, program, pc+1 as usize, relative);
                let operand2 = get_operand(mode2, program, pc+2 as usize, relative);
                let output = get_operand_for_write(mode3, program, pc+3 as usize, relative) as usize;
                ensure_location(program, output as usize);
                program[output] = match operand1 == operand2 {
                    true => 1,
                    false => 0,
                };
                pc += 4;
            },
            Op::REL => {
                let operand = get_operand(mode1, program, pc+1 as usize, relative);
                relative += operand;
                pc += 2;
            }
            Op::HALT => {
                println!("HALT");
                break;
            }
            _ => {
                break;
            }
        };
    }
    panel
}

fn main() {
    let file = File::open("input.txt").expect("Unable to open file input.txt");
    let reader = BufReader::new(file);

    // there should only be one line; ideally this shouldn't be a loop
    // since it's not immediately clear what's going on here
    for line_result in reader.lines() {
        let line = line_result.unwrap();
        let split = line.split(",");

        let mut program = Vec::new();

        for s in split {
            let op = s.parse::<i64>().unwrap();
            program.push(op);
        }

        let mut img = Image::new(5000,5000);
        let panel = run(&mut program);
        println!("{:?} {:?}", panel.min, panel.max);
        let mut count = 0;
        for x in 0..5000 {
            for y in 0..5000 {
                if panel.data[(x + y * panel.width) as usize] == 1 {
                    img.set_pixel(x as u32,y as u32,Pixel{r:255,g:255,b:255});
                }
            }
        }
        let _ = img.save("test.bmp");
        println!("Count: {}", count);
    }
}
