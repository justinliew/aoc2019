use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() {

    let file = File::open("input-1.txt").expect("Unable to open file input.txt");
    let reader = BufReader::new(file);

    let mut total = 0;

    for line in reader.lines() {
        let mass = line.unwrap().parse::<i32>().unwrap();
        // part 1
        // let sum = mass / 3 - 2;
        // total += sum;

        // part 2
        let mut sum = mass / 3 - 2;
        while sum > 0 {
            total += sum;
            sum = sum / 3 - 2;
        }
        // println!("{}", total);
        // std::process::exit(0);
    }
    println!("{}", total);
}
