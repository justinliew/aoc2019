use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let mut layers = Vec::new();
    let mut layer = Vec::new();

    let w = 25;
    let h = 6;

    for (i,c) in contents.chars().enumerate() {
        if i % (w*h) == 0 && i > 0 {
            let new_layer = layer.clone();
//            println!("New Layer: {:?}", new_layer);
            layers.push(new_layer);
            layer.clear();
        }
        let num = c.to_digit(10).unwrap();
        layer.push(num);
    }
    let new_layer = layer.clone();
    layers.push(new_layer);

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
//    println!("The fewest 0 digits is {} at layer {}", num_0, min_layer);

    let mut num_1 = 0;
    let mut num_2 = 0;
    for pixel in &layers[min_layer] {
        match pixel {
            1 => {num_1 += 1},
            2 => {num_2 += 2},
            _ => (),
        }
    }
    // TODO - for some reason this doesn't actually print the correct number
    // I used the debug print of the layer to actually calculate this. It seems to be `num_2` that's wrong
//    println!("{} * {} = {}", num_1, num_2,num_1 * num_2);

    let mut post = vec![255;w*h];
    for i in 0..(w*h) {
        for j in 0..layers.len() {
//            println!("{} {} is {}", i,j,layers[j][i]);
            if layers[j][i] != 2 {
                post[i] = layers[j][i];
                break;
            }
        }
    }

    for y in 0..h {
        for x in 0..w {
            print!("{} ",post[x+y*w]);
        }
        println!("");
    }
}
