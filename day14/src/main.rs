use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Default,Debug,Clone)]
struct Ingredient {
    amt: i32,
    name: String,
}

impl Ingredient {
    fn new(s: &str) -> Self {
        let parts : Vec<&str> = s.split(" ").collect();
        Ingredient{
            amt: parts[0].parse::<i32>().unwrap(),
            name: parts[1].to_string(),
        }
    }
}

#[derive(Debug)]
struct Node {
    cur: Ingredient,
    from: Vec<Node>,
}

impl Node {
    fn new(p: &Production) -> Self {
        let mut from = Vec::new();
        for i in &p.input {
            from.push(Node{
                cur: i.clone(),
                from: Vec::new(),
            });
        }
        Self {
            cur: p.output.clone(),
            from: from,
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

fn get_production(rules: &Vec<Production>, name: &str) -> Option<&Production> {
    rules.iter().find(|&p) p.output.name == name)
}

fn to_node(p: &Production) -> Node {
    Node::new(p)
}

// Node has a current ingredient, and a list of from nodes which are the inputs
// we also need to know the number of output units to produce.
fn run(n: &mut Node, out_amt: i32) {
    for f in n.from {
    }
}

fn main() {
    let file = File::open("input0.txt").expect("Unable to open file input.txt");
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

    let root_rule = get_production(&rules, "FUEL");
    match root_rule {
        None => (),
        Some(p) => {
            let mut root = to_node(p);
            run(&mut root, 1);
        }
    };
}
