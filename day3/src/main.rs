use std::fs::File;
use std::io::{prelude::*, BufReader};

type Point = (i32,i32);

fn length(a: &[Point]) -> i32 {
    (a[0].0 - a[1].0).abs() +
    (a[0].1 - a[1].1).abs()
}

fn traverse(a: &[Point], b: &[Point], aindex: usize, bindex: usize) -> i32 {
    let mut total = 0;
    for ai in 0..aindex {
        let len = length(&a[ai..=(ai+1)]);
        total += len;
    }
    for bi in 0..bindex {
        let len = length(&b[bi..=(bi+1)]);
        total += len;
    }
    let (valid,point) = intersects((a[aindex],a[aindex+1]),(b[bindex],b[bindex+1]));
    total + match valid {
        true => {
            length(&vec![a[aindex],point]) +
            length(&vec![b[bindex],point])
        },
        false => {
            0
        }
    }
}

#[derive(PartialEq,Debug)]
enum Dir {
    Vertical,
    Horizontal,
}

fn intersects(a: (Point,Point), b: (Point,Point)) -> (bool,Point) {
    let a_dir = match (a.0).0 == (a.1).0 {
        true => (Dir::Vertical,(a.0).0,((a.0).1,(a.1).1)),
        false => (Dir::Horizontal,(a.0).1,((a.0).0,(a.1).0))
    };
    let b_dir = match (b.0).0 == (b.1).0 {
        true => (Dir::Vertical,(b.0).0,((b.0).1,(b.1).1)),
        false => (Dir::Horizontal,(b.0).1,((b.0).0,(b.1).0))
    };

    if a_dir.0 == b_dir.0 {
        return (false,(0,0));
    }

    let (al,ag) = match (a_dir.2).0 < (a_dir.2).1 {
        true => ((a_dir.2).0,(a_dir.2).1),
        false => ((a_dir.2).1,(a_dir.2).0),
    };
    let (bl,bg) = match (b_dir.2).0 < (b_dir.2).1 {
        true => ((b_dir.2).0,(b_dir.2).1),
        false => ((b_dir.2).1,(b_dir.2).0),
    };

    let inter = (a_dir.1 >= bl && a_dir.1 <= bg) &&
                (b_dir.1 >= al && b_dir.1 <= ag) &&
                (a_dir.1 != 0 || al != 0 || ag != 0) &&
                (b_dir.1 != 0 || bl != 0 || bg != 0) &&
                (a_dir.1.abs() != 0 && b_dir.1.abs() != 0);
    match a_dir.0 {
        Dir::Vertical => {
            (inter,(a_dir.1,b_dir.1))
        },
        _ => {
            (inter,(b_dir.1,a_dir.1))
        }
    }
}

fn main() {
    let file = File::open("input.txt").expect("Unable to open file input.txt");
    let reader = BufReader::new(file);

    let mut segs = Vec::new();

    // change this to split on comma
    for line_result in reader.lines() {
        let mut seg = Vec::new();
        let line = line_result.unwrap();
        let split = line.split(",");

        let mut last = (0,0);
        seg.push(last);

        for s in split {
            let dir = s.chars().nth(0).unwrap();
            let amt = s[1..].parse::<i32>().unwrap();
            let cur = match dir {
                'R' => {
                    (last.0+amt,last.1)
                },
                'L' => {
                    (last.0-amt,last.1)
                },
                'U' => {
                    (last.0,last.1+amt)
                },
                'D' => {
                    (last.0,last.1-amt)
                },
                _ => {
                    (0,0) // error
                }
            };
            seg.push(cur);
            last = cur;
        }
        segs.push(seg);
    }

    let mut smallest = std::i32::MAX;
    for i in 0..segs[0].len()-1 {
        for j in 0..segs[1].len()-1 {
            let (valid,_) = intersects((segs[0][i],segs[0][i+1]),(segs[1][j],segs[1][j+1]));
            if valid {
                // part 1
                // let dist = point.0.abs() + point.1.abs();
                // if dist != 0 && dist < smallest {
                //     smallest = point.0.abs() + point.1.abs();
                // }

                // part 2
                let dist = traverse(&segs[0], &segs[1],i,j);
                if dist != 0 && dist < smallest {
                    smallest = dist;
                }
            }
        }
    }
    println!("Smallest distance: {}", smallest);
}