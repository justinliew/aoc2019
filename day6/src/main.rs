use std::fs::File;
use std::io::{prelude::*, BufReader};

type NodeIndex = usize;

struct Node {
    name: String,
    index: usize,
    target: Option<NodeIndex>,
}

struct Graph {
    nodes: Vec<Node>,
}

impl Graph {

    fn new() -> Graph {
        Graph{
            nodes: Vec::new(),
        }
    }
    fn add_or_get_node(&mut self, name: String) -> NodeIndex {

        for (index,node) in self.nodes.iter().enumerate() {
            if node.name == name {
                return index;
            }
        }

        let index = self.nodes.len();
        self.nodes.push(Node {name: name, index: index, target: None });
        index
    }

    fn add_edge(&mut self, source: NodeIndex, target: NodeIndex) {
        self.nodes[source].target = Some(target);
    }

    fn get_parent(&self, source: NodeIndex) -> Option<NodeIndex> {
        self.nodes[source].target
    }

    fn get_node(&self, name: &str) -> Option<NodeIndex> {
        for i in 0..self.nodes.len() {
            if self.nodes[i].name == name {
                return Some(self.nodes[i].index);
            }
        }
        None
    }
}

fn main() {
    let file = File::open("input.txt").expect("Unable to open file input.txt");
    let reader = BufReader::new(file);

    let mut graph = Graph::new();
    for line_result in reader.lines() {
        let line = line_result.unwrap();
        let split : Vec<&str> = line.split(")").collect();
        let child_index = graph.add_or_get_node(split[1].to_string());
        let parent_index = graph.add_or_get_node(split[0].to_string());
        graph.add_edge(child_index, parent_index);
    }

// part 1
//     let mut orbits = 0;
//     for i in 0..graph.nodes.len() {
//         let mut pi = i;
//         loop {
//             let parent = graph.get_parent(pi);
//             pi = match parent {
//                 None => break,
//                 Some(n) => {
//                     orbits += 1;
// //                    println!("{} is a parent of {}", graph.nodes[n].name, graph.nodes[i].name);
//                     n
//                 },
//             }
//         }
//     }
//     println!("Number of orbits: {}", orbits);

// part 2 - just hard code the names
    let you = graph.get_node("YOU");
    let san = graph.get_node("SAN");
    let mut you_parents = Vec::new();
    let mut san_parents = Vec::new();

    let mut pi = you.unwrap();
    loop {
        let parent = graph.get_parent(pi);
        pi = match parent {
            None => break,
            Some(n) => {
                n
            },
        };
        you_parents.push(parent);
    }

    pi = san.unwrap();
    loop {
        let parent = graph.get_parent(pi);
        pi = match parent {
            None => break,
            Some(n) => {
                n
            },
        };
        san_parents.push(parent);
    }

    'outer: for yi in 0..you_parents.len() {
        for si in 0..san_parents.len() {
            if san_parents[si] == you_parents[yi] {
                println!("Common ancestor {} is {} hops away",
                            san_parents[si].unwrap(),
                            yi + si);
                            break 'outer;
            }
        }
    }
}
