use std::fs::File;
use std::io::{prelude::*, BufReader};
//use std::collections::HashMap;
use std::collections::BTreeMap;

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

// fn approx_eq(a: f32, b: f32, epsilon: f32) -> bool {
//     (a - b).abs() < epsilon
// }

// fn dir_approx_eq(a: &Normalized, b: &Normalized) -> bool {
//     approx_eq(a.x,b.x,0.001) && approx_eq(a.y,b.y,0.001)
// }

// fn slope_and_direction(a: &Asteroid, b: &Asteroid, d: bool) -> (f32, Normalized) {
//     let len = (((b.x-a.x).pow(2) + (b.y-a.y).pow(2)) as f32).sqrt();
//     if d {
//         println!("Length: {} for {}-{} and {}-{}", len, b.x,a.x,b.y,a.y);
//     }
//     ((a.y-b.y) as f32 / (a.x-b.x) as f32,
//     Normalized
//     {
//         x:((b.x-a.x) as f32)/len,
//         y:((b.y-a.y) as f32)/len
//     })
// }

// fn collinear(a: &Asteroid, b: &Asteroid,c: &Asteroid, d: bool) -> bool {
//     let (sa,da) = slope_and_direction(a,c,d);
//     let (sb,db) = slope_and_direction(b,c,d);
//     if d {
//         println!("Slopes are: {}/{:?} and {}/{:?} {}", sa,da,sb,db, approx_eq(sa,sb,0.001));
//     }
//     ((!sa.is_finite() && !sb.is_finite()) || approx_eq(sa,sb,0.001)) && !dir_approx_eq(&da,&db)
// }

#[derive(PartialEq,Eq,PartialOrd,Ord,Debug)]
struct FloatKey {
    integral: i32,
    fractional: u32,
}

fn f32_to_key(f: f32) -> FloatKey {
    let i = f as i32;
    FloatKey{
        integral: i,
        fractional: ((f-(i as f32)) * 1000.) as u32
    }
}

fn angle_between(root: &Asteroid, p: &Asteroid) -> f32 {
    let vec = Asteroid{
        x: p.x-root.x,
        y: root.y-p.y
    };
    let mag = ((vec.x*vec.x + vec.y*vec.y) as f32).sqrt();
    let norm = Normalized{
        x: vec.x as f32 / mag,
        y: vec.y as f32 / mag,
    };

    let dot = norm.y as f32;
    let det = norm.x as f32;
    let angle = det.atan2(dot);
    println!("{:?}  -> {:?} -> {} -> {}, {}, {}", p, vec, dot, dot.acos(), det, angle);
    if det < 0.0 {
        std::f32::consts::PI * 2.0 + angle
    } else {
        angle
    }
    // if vec.x >= 0 {
    //     dot.acos()
    // } else {
    //     2.0 * std::f32::consts::PI - dot.acos()
    // }
}

fn distance(a: &Asteroid, b: &Asteroid) -> f32 {
    let vec = Asteroid{
        x: b.x-a.x,
        y: b.y-a.y
    };
    ((vec.x*vec.x + vec.y*vec.y) as f32).sqrt()
}

// 26, 29 is the station for part 1

// 186 low
// 725,729 high

fn main() {
    let file = File::open("input.txt").expect("Unable to open file input.txt");
    let reader = BufReader::new(file);

    let mut input = Vec::new();
    let mut height = 0;
    let mut asteroids = Vec::new();
    let mut laser = Asteroid{x:0,y:0};

    for line in reader.lines() {
        let mut width = 0;
        for point in line.unwrap().chars() {
            match point {
                '#' => {
                    input.push(1);
                    asteroids.push(Asteroid{x: width, y: height,});
                },
                '.' => {
                    input.push(0);
                },
                'X' => {
                    input.push(2);
                    laser = Asteroid{x: width, y: height};
                }
                _ => (),
            }
            width += 1;
        }
        height += 1;
    }

    // part 1
//   let mut sights = HashMap::new();
//     for a in &asteroids {
//         for b in &asteroids {
// //            let do_debug = a.x == 3 && a.y == 2;
//             let do_debug = false;
//             let mut los = true;
//             for c in &asteroids {
//                 if a != b && b != c && a != c {
//                     if do_debug {
//                         let t = collinear(a,b,c, true);
//                         if t {
//                             println!("Comparing {:?} and {:?} to {:?} with {}", a,b, c, t);
//                         }
//                     }
//                     if collinear(a,b,c, false) {
//                         los = false;
//                         break;
//                     }
//                 }
//             };
//             if a != b && los {
//                 let sight_a = sights.entry(a).or_insert(0);
//                 *sight_a += 1;
//                 if do_debug{
//                     println!("{:?} and {:?} can see", a,b);
//                 }
//             }
//         }
//     }

//     let mut most = 0;
//     let mut station = Asteroid{x:0,y:0};
//     for (a,s) in sights {
//         println!("{:?} has {:?}",a,s);
//         if s > most {
//             most = s;
//             station.x = a.x;
//             station.y = a.y;
//         }
//     }
//     println!("The most is {}, at {:?}", most,station);

    // part 2
    // we need to sort vector by angle, and keep the asteroids sorted by distance
    let mut sorted_by_angle = BTreeMap::new();

//  //                 let sight_a = sights.entry(a).or_insert(0);

    for a in &asteroids {
        // calculate angle
        let angle = angle_between(&laser, a);
        let key = f32_to_key(angle);
        let d = distance(&laser,a);
        let dkey = f32_to_key(d);

        let l = sorted_by_angle.entry(key).or_insert(BTreeMap::new());
        l.insert(dkey,a);
    }

    // find nth
    // number of angles
    // number of asteroids in each angle, sorted lowest to highest
    // increment
    let mut max = 200;
    for cur in 0..max {
        let mut nth = cur;
    //    let unique_angles = sorted_by_angle.len();
    //    println!("We have {} angles", unique_angles);
        let mut buckets = Vec::new();
        let mut i = 0;
        for (_,points) in &sorted_by_angle {
            buckets.push((points.len(),i));
            i += 1;
        }
        buckets.sort();
        // println!("ANGLES");
        // for angle in &sorted_by_angle {
        //     println!("{:?}", angle);
        // }
    //    println!("Buckets: {:?}", buckets);

        while nth >= 0 {
            let bucket_len = buckets[0].0;
            let chunk = buckets[0].0 * buckets.len();
            if nth < chunk { // if we are off by 1 then check this

                // get the sorted bucket index
                let bucket_idx = nth % buckets.len();
                // then get the entry there
                let (_, abucket) = sorted_by_angle.iter().nth(bucket_idx).unwrap();
                let a = abucket.iter().nth(nth / buckets.len()).unwrap();
                println!("We found: {:?} for {}", a, cur);
                break;
            }
            while buckets[0].0 == bucket_len {
                buckets.remove(0);
            }
            nth -= chunk;
        }
    }
}
