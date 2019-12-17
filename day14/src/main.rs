use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Default,Debug)]
struct Ingredient {
    amt: i32,
    name: String,
    opt_output: String,
}

impl Ingredient {
    fn new(s: &str) -> Self {
        let parts : Vec<&str> = s.split(" ").collect();
        Ingredient{
            amt: parts[0].parse::<i32>().unwrap(),
            name: parts[1].to_string(),
            opt_output: "".to_string(),
        }
    }
}

#[derive(Debug)]
struct Production {
    input: Vec<Ingredient>,
    output: Ingredient,
}

impl Production {
    fn new() -> Self {
        Production{
            input: Vec::new(),
            output: Default::default(),
        }
    }

    fn add_input(&mut self, i: Ingredient) {
        self.input.push(i);
    }

    fn set_output(&mut self, o: Ingredient) {
        self.output = o;
    }
}

fn push_or_update(intermediates: &mut Vec<Ingredient>, name: &str, amt: i32, opt_output: &str) {

    if name == "ORE" {
        println!("PUSHING ORE");
        for i in 0..intermediates.len() {
            if intermediates[i].name == name && intermediates[i].opt_output == opt_output {
                println!("We found ore made by {}", opt_output);
                intermediates[i].amt += amt;
                return;
            }
        }
    } else {
        for i in 0..intermediates.len() {
            if intermediates[i].name == name {
                intermediates[i].amt += amt;
                return;
            }
        }
    }
    intermediates.push(Ingredient{amt:amt, name: name.to_string(), opt_output: opt_output.to_string()});

}

fn main() {
    let file = File::open("input1.txt").expect("Unable to open file input.txt");
    let reader = BufReader::new(file);

    let mut rules = Vec::new();

    for line_result in reader.lines() {
        let line = line_result.unwrap();
        let io_split : Vec<&str> = line.split(" => ").collect();
        let in_split : Vec<&str> = io_split[0].split(", ").collect();
        // io_split[1]

        let mut p = Production::new();

        for i in in_split {
            p.add_input(Ingredient::new(i));
        }
        p.set_output(Ingredient::new(io_split[1]));
        rules.push(p);
    }

    // for r in &rules {
    //     println!("{:?}", r);
    // }

    // find the FUEL, work backwards until we find only ORE

    // when we consume using a rule that includes ORE as the input, we need to push that to a separate list

    let mut intermediates = Vec::new();
    push_or_update(&mut intermediates, "FUEL", 1, "");

    let mut ore = 0;
    let mut ore_ingredients : Vec<Ingredient> = Vec::new();
    while intermediates.len() > 0 {
        let ing = intermediates.pop().unwrap();
        println!("Producing with {} {}", ing.amt, ing.name);

        match rules.iter().find(|&r| r.output.name == ing.name) {
            None => {
                println!("Using {} ORE", ing.amt);
                ore += ing.amt;
            },
            Some(production) => {
                for i in &production.input {
                    if i.name == "ORE" {
                        // we need to find ore_ingredients and subtract as much as we need to first
                        let mut rem = ing.amt;
                        for i in 0..ore_ingredients.len() {
                            let idx = i as usize;
                            if ore_ingredients[idx].opt_output == production.output.name {
                                match ore_ingredients[idx].amt - rem {
                                    // we have more than we need; just subtract all
                                    i if i > 0 => {
                                        ore_ingredients[idx].amt -= rem;
                                        rem = 0;
                                    },
                                    // we have not enough
                                    i if i < 0 => {
                                        rem -= ore_ingredients[idx].amt;
                                        ore_ingredients[idx].amt = 0;
                                    },
                                    // we have the exact amount
                                    _ => {
                                        rem = 0;
                                        ore_ingredients[idx].amt = 0;
                                    }
                                }
                            }
                        }

                        if rem > 0 {
                            let mut mult = 0;
                            while rem > 0 {
                                rem -= production.output.amt;
                                mult += 1;
                            }
                            rem = rem.abs();

                            println!("Found ORE; consumed {} to make {} {}. {} leftover", i.amt*mult, production.output.amt * mult, production.output.name, rem);
                            ore += i.amt*mult;

                            let mut found = false;

                            // we need to add -rem to the ore_ingredients
                            for i in 0..ore_ingredients.len() {
                                if ore_ingredients[i].opt_output == production.output.name && rem > 0 {
                                    println!("Adding {} {} to excess", production.output.name, rem);
                                    ore_ingredients[i].amt += rem;
                                    found = true;
                                    break;
                                }
                            }

                            if !found {
                                println!("Adding {} {} to excess", production.output.name, rem);
                                ore_ingredients.push(Ingredient{
                                    name: i.name.clone(), // "ORE"
                                    amt: rem,
                                    opt_output: production.output.name.clone(),
                                });
                            }
                        }
                    } else {
                        let mut rem = ing.amt - production.output.amt;
                        let mut mult = 1;
                        while rem > 0 {
                            rem -= production.output.amt;
                            mult += 1;
                        }
                        push_or_update(&mut intermediates, &i.name, i.amt * mult, &production.output.name);
                        if rem < 0 {
                            push_or_update(&mut intermediates, &ing.name, -rem, &production.output.name);
                        }
                    }
                }
            }
        }
    }

    println!("ORE: {}", ore);
}
