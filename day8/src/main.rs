use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let mut layers = Vec::new();
    let mut layer = Vec::new();

    for (i,c) in contents.chars().enumerate() {
        if i % (25*6) == 0 && i > 0 {
            let new_layer = layer.clone();
//            println!("New Layer: {:?}", new_layer);
            layers.push(new_layer);
            layer.clear();
        }
        let num = c.to_digit(10).unwrap();
        layer.push(num);
    }

    let mut num_0 = std::i32::MAX;
    let mut min_layer = 0;
    for (layeridx,layer) in layers.iter().enumerate() {
        let mut cur = 0;
        for pixelidx in 0..layer.len() {
            if layer[pixelidx] == 0 {
                cur += 1;
            }
        }
        if cur < num_0 {
            num_0 = cur;
            min_layer = layeridx;
        }
    }
    println!("The fewest 0 digits is {} at layer {}", num_0, min_layer);

    let mut num_1 = 0;
    let mut num_2 = 0;
    for pixel in &layers[min_layer] {
        print!("{} ", pixel);
        match pixel {
            1 => {num_1 += 1},
            2 => {num_2 += 2},
            _ => (),
        }
    }
    println!("{} * {} = {}", num_1, num_2,num_1 * num_2);

}
