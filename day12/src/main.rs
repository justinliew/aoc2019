use std::ops;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Vec3 {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Vec3 {x:x,y:y,z:z}
    }

    fn unit(&self) -> Self {
        Vec3::new(
            match self.x {
                x if x == 0 => {0},
                x if x > 0 => {1},
                x if x < 0 => {-1},
                _ => 0,
            },
            match self.y {
                y if y == 0 => {0},
                y if y > 0 => {1},
                y if y < 0 => {-1},
                _ => 0,
            },
            match self.x {
                z if z == 0 => {0},
                z if z > 0 => {1},
                z if z < 0 => {-1}
                _ => 0,
            },)
    }
}

// if m2 is greater than m1, then subtracting will make diff negative
// then unit will be -1 and subtracting will add to v1
fn gravity(m1: &Vec3, m2: &Vec3, v1: &mut Vec3) {
    if m2.x > m1.x {
        v1.x += 1;
    }
    if m2.y > m1.y {
        v1.y += 1;
    }
    if m2.z > m1.z {
        v1.z += 1;
    }

    if m2.x < m1.x {
        v1.x -= 1;
    }
    if m2.y < m1.y {
        v1.y -= 1;
    }
    if m2.z < m1.z {
        v1.z -= 1;
    }
}

fn update(p: &mut Vec3, v: &Vec3) {
    p.x += v.x;
    p.y += v.y;
    p.z += v.z;
}

fn energy(p: &Vec3, v: &Vec3) -> i32 {
    (p.x.abs() + p.y.abs() + p.z.abs()) *
    (v.x.abs() + v.y.abs() + v.z.abs())
}

fn main() {
    let file = File::open("input.txt").expect("Unable to open file input.txt");
    let reader = BufReader::new(file);

    // there should only be one line; ideally this shouldn't be a loop
    // since it's not immediately clear what's going on here
    let mut moon_pos = Vec::new();
    let mut moon_vel = Vec::new();
    for line_result in reader.lines() {
        let line = line_result.unwrap();
        let split : Vec<&str> = line.split(",").collect();
        moon_pos.push(Vec3::new(
                split[0].parse::<i32>().unwrap(),
                split[1].parse::<i32>().unwrap(),
                split[2].parse::<i32>().unwrap(),
            ),
        );
        moon_vel.push(Vec3::new(0,0,0));
    }

    // we are doing this twice per pair; maybe we can increase perf by changing the nested for loop...

    for e in 1..=1000 {
        for m1 in 0..moon_pos.len() {
            for m2 in 0..moon_pos.len() {
                gravity(&moon_pos[m1],&moon_pos[m2], &mut moon_vel[m1]);
            }
        }
        for m in 0..moon_pos.len() {
            update(&mut moon_pos[m],&moon_vel[m]);
        }

        println!("Step {}", e);
        let mut total = 0;
        for i in 0..moon_pos.len() {
            let amt = energy(&moon_pos[i],&moon_vel[i]);
            total += amt;
            println!("Moon {:?} has vel {:?} {}", moon_pos[i], moon_vel[i], amt);
        }
        println!("{}", total);
    }
}
