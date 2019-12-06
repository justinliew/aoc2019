use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug)]
enum Op {
    INVALID=0,
    ADD=1,
    MULTIPLY=2,
    STORE=3,
    PRINT=4,
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

fn run(mut program: Vec<i32>, input: i32) -> bool {
    let mut pc = 0;
    while pc < program.len() {

        let (op,mode1,mode2,_) = get_parameters(program[pc]);
        println!("Program: {:?}", &program[pc..]);
        println!("Handling {} -> {:?},{:?},{:?}", program[pc],op,mode1,mode2);

        match op {
            Op::ADD => {
                let operand1 = match mode1 {
                    Mode::POSITION => program[program[pc+1] as usize],
                    Mode::IMMEDIATE => program[pc+1],
                };
                let operand2 = match mode2 {
                    Mode::POSITION => program[program[pc+2] as usize],
                    Mode::IMMEDIATE => program[pc+2],
                };
                let output = program[pc+3] as usize;
                println!("Adding {} and {}", operand1, operand2);
                program[output] = operand1 + operand2;
                pc += 4;
            },
            Op::MULTIPLY => {
                let operand1 = match mode1 {
                    Mode::POSITION => program[program[pc+1] as usize],
                    Mode::IMMEDIATE => program[pc+1],
                };
                let operand2 = match mode2 {
                    Mode::POSITION => program[program[pc+2] as usize],
                    Mode::IMMEDIATE => program[pc+2],
                };
                let output = program[pc+3] as usize;
                println!("Multiplying {} and {}", operand1, operand2);
                program[output] = operand1 * operand2;
                pc += 4;
            },
            Op::STORE => {
                let output = program[pc+1] as usize;
                program[output] = input;
                pc += 2;
            },
            Op::PRINT => {
                println!("{}", program[program[pc+1] as usize]);
                pc += 2;
            }
            Op::HALT => {
                break;
            }
            _ => {
                break;
            }
        };
    }
    true // ??
}

fn main() {
    let input = 1;
    let file = File::open("input.txt").expect("Unable to open file input.txt");
    let reader = BufReader::new(file);

    // change this to split on comma
    for line_result in reader.lines() {
        let line = line_result.unwrap();
        let split = line.split(",");

        let mut program = Vec::new();

        for s in split {
            let op = s.parse::<i32>().unwrap();
            program.push(op);
        }

        if run(program.clone(),input) {
        }
    }
}
