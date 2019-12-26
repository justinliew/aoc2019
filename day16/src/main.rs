use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::iter;
use std::char;

fn dot(base: &[i32], pattern: &[i32]) -> i32 {
    let t = base.iter().zip(pattern.iter());
//    println!("T: {:?}", t);
    let m =   t.map(|(a,b)| a*b).sum::<i32>()
                    .abs()
                    .to_string()
                    .chars()
                    .rev()
                    .next()
                    .unwrap()
                    .to_digit(10)
                    .unwrap();
    m as i32
}

fn calc_pattern(base: &[i32], rep: usize, len: usize) -> Vec<i32> {
    let mut ret : Vec<i32> = Vec::new();
    let mut total = 0;

    'outer: loop {
        for b in base {
            for _i in 0..rep {
                ret.push(*b);
                total += 1;
                if total > len {
                    break 'outer;
                }
            }
        }
    }
    ret[1..].to_vec()
}

fn main() {
    let file = File::open("input.txt").expect("Unable to open file input.txt");
    let reader = BufReader::new(file);

    let base = vec![0,1,0,-1];

    for line_result in reader.lines() {
        let line = line_result.unwrap();

        let original : Vec<i32> = line.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();

        let num_phases = 100;

        let mut output = Vec::new();
        let mut input = original.clone();
        for iteration in 0..num_phases {

            // we need to iterate over each input element and construct an output element
            for (idx,i) in input.iter().enumerate() {
                let pattern = calc_pattern(&base, idx+1, input.len());
                output.push(dot(&input,&pattern));

//                println!("Pattern is {:?}", pattern);
            }
            input = output.clone();
            println!("Output: {:?}", output);
            output.clear();
        }
    }
}
