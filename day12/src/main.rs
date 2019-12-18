use std::ops;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::hash::{Hash, Hasher};
use std::time::{Duration,Instant};
use std::collections::HashSet;
use fasthash::xx::Hasher64;

#[derive(Debug,Hash)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Vec3 {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Vec3 {x:x,y:y,z:z}
    }

    // fn unit(&self) -> Self {
    //     Vec3::new(
    //         match self.x {
    //             x if x == 0 => {0},
    //             x if x > 0 => {1},
    //             x if x < 0 => {-1},
    //             _ => 0,
    //         },
    //         match self.y {
    //             y if y == 0 => {0},
    //             y if y > 0 => {1},
    //             y if y < 0 => {-1},
    //             _ => 0,
    //         },
    //         match self.x {
    //             z if z == 0 => {0},
    //             z if z > 0 => {1},
    //             z if z < 0 => {-1}
    //             _ => 0,
    //         },)
    // }
}

// if m2 is greater than m1, then subtracting will make diff negative
// then unit will be -1 and subtracting will add to v1
fn gravity(m1: &Vec3, m2: &Vec3, v: &mut [Vec3], v1: usize, v2: usize) {
    if m2.x > m1.x {
        v[v1].x += 1;
        v[v2].x -= 1;
    }
    if m2.y > m1.y {
        v[v1].y += 1;
        v[v2].y -= 1;
    }
    if m2.z > m1.z {
        v[v1].z += 1;
        v[v2].z -= 1;
    }

    if m2.x < m1.x {
        v[v1].x -= 1;
        v[v2].x += 1;
    }
    if m2.y < m1.y {
        v[v1].y -= 1;
        v[v2].y += 1;
    }
    if m2.z < m1.z {
        v[v1].z -= 1;
        v[v2].z += 1;
    }
}

fn update(p: &mut Vec3, v: &Vec3) {
    p.x += v.x;
    p.y += v.y;
    p.z += v.z;
}

// fn energy(p: &Vec3, v: &Vec3) -> i32 {
//     (p.x.abs() + p.y.abs() + p.z.abs()) *
//     (v.x.abs() + v.y.abs() + v.z.abs())
// }

// fn calculate_hash<T: Hash>(t1: &T, t2: &T) -> u64 {
//     let mut s = Hasher64::default();
//     t1.hash(&mut s);
//     t2.hash(&mut s);
//     s.finish()
// }

fn main() {
    let file = File::open("input0.txt").expect("Unable to open file input.txt");
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

     let mut e : u64 = 0;
    'outer: loop {
        for m1 in 0..moon_pos.len() {
            for m2 in (m1+1)..moon_pos.len() {
                gravity(&moon_pos[m1],&moon_pos[m2], &mut moon_vel, m1, m2);
            }
        }

        for m in 0..moon_pos.len() {
            update(&mut moon_pos[m],&moon_vel[m]);
        }
        // for m in 0..moon_pos.len() {
        //     println!("{:?} {:?}", moon_pos[m], moon_vel[m]);
        // }

        let mut found = true;
        for m in &moon_vel {
            if m.x != 0 || m.y != 0 || m.z != 0 {
                found = false;
            }
        }
        if found {
            println!("We are at rest at iteration {}", e);
        }
        e += 1;
    }
}
