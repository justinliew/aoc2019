use std::fs::File;
use std::io::{prelude::*, BufReader};

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
    HALT=99,
}

#[derive(Debug)]
enum Mode {
    POSITION = 0,
    IMMEDIATE = 1,
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
        99 => Op::HALT,
        _ => Op::INVALID,
    },
    match (op / 100) % 10 {
        0 => Mode::POSITION,
        1 => Mode::IMMEDIATE,
        _ => Mode::POSITION,
    },
    match (op / 1000) % 10 {
        0 => Mode::POSITION,
        1 => Mode::IMMEDIATE,
        _ => Mode::POSITION,
    },
    match (op / 10000) % 10 {
        0 => Mode::POSITION,
        1 => Mode::IMMEDIATE,
        _ => Mode::POSITION,
    })
}

fn get_operand(mode: Mode, program: &[i32], pc: usize) -> i32 {
    match mode {
        Mode::POSITION => program[program[pc] as usize],
        Mode::IMMEDIATE => program[pc],
    }
}

fn run(mut program: Vec<i32>, input: &[i32]) -> i32 {
    let mut pc = 0;
    let mut ic = 0;
    let mut output = 0;
    while pc < program.len() {

        let (op,mode1,mode2,_) = get_parameters(program[pc]);
        // println!("Program: {:?}", &program[pc..]);
        // println!("({})Handling {} -> {:?},{:?},{:?}", pc, program[pc],op,mode1,mode2);

        match op {
            Op::ADD => {
                let operand1 = get_operand(mode1, &program, pc+1 as usize);
                let operand2 = get_operand(mode2, &program, pc+2 as usize);
                let output = program[pc+3] as usize;
                program[output] = operand1 + operand2;
                pc += 4;
            },
            Op::MULTIPLY => {
                let operand1 = get_operand(mode1, &program, pc+1 as usize);
                let operand2 = get_operand(mode2, &program, pc+2 as usize);
                let output = program[pc+3] as usize;
                program[output] = operand1 * operand2;
                pc += 4;
            },
            Op::STORE => {
                let output = program[pc+1] as usize;
                program[output] = input[ic];
                ic += 1;
                pc += 2;
            },
            Op::PRINT => {
                let operand1 = get_operand(mode1, &program, pc+1 as usize);
//                println!("{}", operand1);
                output = operand1;
                pc += 2;
            },
            Op::JT => {
                let operand1 = get_operand(mode1, &program, pc+1 as usize);
                let operand2 = get_operand(mode2, &program, pc+2 as usize);
                if operand1 != 0 {
                    pc = operand2 as usize;
                } else {
                    pc += 3;
                }
            },
            Op::JF => {
                let operand1 = get_operand(mode1, &program, pc+1 as usize);
                let operand2 = get_operand(mode2, &program, pc+2 as usize);
                if operand1 == 0 {
                    pc = operand2 as usize;
                } else {
                    pc += 3;
                }
            },
            Op::LT => {
                let operand1 = get_operand(mode1, &program, pc+1 as usize);
                let operand2 = get_operand(mode2, &program, pc+2 as usize);
                let output = program[pc+3] as usize;
                program[output] = match operand1 < operand2 {
                    true => 1,
                    false => 0,
                };
                pc += 4;
            },
            Op::EQ => {
                let operand1 = get_operand(mode1, &program, pc+1 as usize);
                let operand2 = get_operand(mode2, &program, pc+2 as usize);
                let output = program[pc+3] as usize;
                program[output] = match operand1 == operand2 {
                    true => 1,
                    false => 0,
                };
                pc += 4;
            },
            Op::HALT => {
                break;
            }
            _ => {
                break;
            }
        };
    }
    output
}

fn perm5() -> Vec<Vec<i32>> {
    let mut ret = Vec::new();

    let mut c = [0; 5];
    let mut base = vec![0,1,2,3,4];

    ret.push(base.clone());

    let mut i = 0;
    while i < 5 {
        if c[i] < i {
            if i % 2 == 0 {
                let temp = base[0];
                base[0] = base[i];
                base[i] = temp;
            } else {
                let temp = base[c[i]];
                base[c[i]] = base[i];
                base[i] = temp;
            }
            ret.push(base.clone());
            c[i] += 1;
            i = 0;
        } else {
            c[i] = 0;
            i += 1;
        }
    }

    ret
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
            let op = s.parse::<i32>().unwrap();
            program.push(op);
        }

        let phases = perm5();
//        println!("{:?}", phases);

        let mut max_output = 0;
        for phase in phases {
            let num_amps = 5;
            let mut cur = 0;
            for a in 0..num_amps {
                cur = run(program.clone(), &[phase[a],cur]);
                if cur > max_output {
                    max_output = cur;
                }
            }
        }
        println!("Max output is {}", max_output);
    }
}
