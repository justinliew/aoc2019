use std::fs::File;
use std::io::{prelude::*, BufReader};

type NodeIndex = usize;

struct Node {
    name: String,
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
        self.nodes.push(Node {name: name, target: None });
        index
    }

    fn add_edge(&mut self, source: NodeIndex, target: NodeIndex) {
        self.nodes[source].target = Some(target);
    }

    fn get_parent(&self, source: NodeIndex) -> Option<NodeIndex> {
        self.nodes[source].target
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
    let mut orbits = 0;
    for i in 0..graph.nodes.len() {
        let mut pi = i;
        loop {
            let parent = graph.get_parent(pi);
            pi = match parent {
                None => break,
                Some(n) => {
                    orbits += 1;
//                    println!("{} is a parent of {}", graph.nodes[n].name, graph.nodes[i].name);
                    n
                },
            }
        }
    }
    println!("Number of orbits: {}", orbits);
}