use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
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

        let mut pc = 0;
        while pc < program.len() {
            println!("Op: {}", program[pc]);
            let (i,v) = match program[pc] {
                1 => {
                    (program[pc+3],program[program[pc+1] as usize]+program[program[pc+2] as usize])
                }
                2 => {
                    (program[pc+3],program[program[pc+1] as usize]*program[program[pc+2] as usize])
                }
                99 => {
                    (-1,0)
                }
                _ => {
                    (-1,0)
                }
            };
            println!("i,v: {},{}", i,v);
            if i >= 0 {
                program[i as usize] = v;
                pc += 4;
            } else {
                break;
            }
        }
        println!("{}", program[0]);
    }
}
