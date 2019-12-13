use std::fs::File;
use std::io::{prelude::*, BufReader};
// extern crate bmp;
// use bmp::{Image,Pixel};

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

fn run(program: &mut ProgramType) {
    let mut relative : i64 = 0;
    let mut pc: PcType = 0;

    let mut idx = 0;

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
                write_location(program,output_idx as usize, 0);
                println!("STORE?");
                pc += 2;
            },
            Op::PRINT => {
                let operand1 = get_operand(mode1, program, pc+1 as usize, relative);
                if (idx+1) % 3 == 0 {
                    println!("{}", operand1);
                }
//                println!("{}", operand1);
                pc += 2;
                idx += 1;
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

        run(&mut program);
    }
}
