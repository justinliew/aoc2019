use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;

#[derive(Hash,Eq,PartialEq,Debug)]
struct Asteroid {
    x: i32,
    y: i32,
}

#[derive(PartialEq,Debug)]
struct Normalized {
    x: f32,
    y: f32,
}

fn approx_eq(a: f32, b: f32, epsilon: f32) -> bool {
    (a - b).abs() < epsilon
}

fn dir_approx_eq(a: &Normalized, b: &Normalized) -> bool {
    approx_eq(a.x,b.x,0.001) && approx_eq(a.y,b.y,0.001)
}

fn slope_and_direction(a: &Asteroid, b: &Asteroid, d: bool) -> (f32, Normalized) {
    let len = (((b.x-a.x).pow(2) + (b.y-a.y).pow(2)) as f32).sqrt();
    if d {
        println!("Length: {} for {}-{} and {}-{}", len, b.x,a.x,b.y,a.y);
    }
    ((a.y-b.y) as f32 / (a.x-b.x) as f32,
    Normalized
    {
        x:((b.x-a.x) as f32)/len,
        y:((b.y-a.y) as f32)/len
    })
}

fn collinear(a: &Asteroid, b: &Asteroid,c: &Asteroid, d: bool) -> bool {
    let (sa,da) = slope_and_direction(a,c,d);
    let (sb,db) = slope_and_direction(b,c,d);
    if d {
        println!("Slopes are: {}/{:?} and {}/{:?} {}", sa,da,sb,db, approx_eq(sa,sb,0.001));
    }
    ((!sa.is_finite() && !sb.is_finite()) || approx_eq(sa,sb,0.001)) && !dir_approx_eq(&da,&db)
}

fn main() {
    let file = File::open("input.txt").expect("Unable to open file input.txt");
    let reader = BufReader::new(file);

    let mut input = Vec::new();
    let mut width = 0;
    let mut height = 0;
    let mut Asteroids = Vec::new();

    for line in reader.lines() {
        width = 0;
        for point in line.unwrap().chars() {
            match point {
                '#' => {
                    input.push(1);
                    Asteroids.push(Asteroid{x: width, y: height,});
                },
                '.' => {
                    input.push(0);
                },
                _ => (),
            }
            width += 1;
        }
        height += 1;
    }

    let mut sights = HashMap::new();

    for a in &Asteroids {
        for b in &Asteroids {
            let do_debug = a.x == 3 && a.y == 2;
            let mut los = true;
            for c in &Asteroids {
                if a != b && b != c && a != c {
                    if do_debug {
                        let t = collinear(a,b,c, true);
                        if t {
                            println!("Comparing {:?} and {:?} to {:?} with {}", a,b, c, t);
                        }
                    }
                    if collinear(a,b,c, false) {
                        los = false;
                        break;
                    }
                }
            };
            if a != b && los {
                let sight_a = sights.entry(a).or_insert(0);
                *sight_a += 1;
                if do_debug{
                    println!("{:?} and {:?} can see", a,b);
                }
            }
        }
    }

    let mut most = 0;
    for (a,s) in sights {
        println!("{:?} has {:?}",a,s);
        if s > most {
            most = s;
        }
    }
    println!("The most is {}", most);

    // for x in 0..width {
    //     for y in 0..height {
    //         if input[x + y * width] == 1 {

    //         }
    //     }
    // }
}
