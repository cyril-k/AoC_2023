use std::collections::{HashMap, HashSet};

fn main () {
    let input = include_str!("./part1.txt");
    let output = part1(input);
    println!("{output}")
}

fn part1(input: &str) -> String {
    let mut graph:HashMap<String, HashSet<String>> = HashMap::new();

    for line in input.lines() {
        let splits: Vec<_> = line.split(": ").collect();

        let node = splits[0].to_owned();
        let neighbor_nodes = splits[1]
            .split_whitespace()
            .map(|s| s.to_owned())
            .collect::<Vec<_>>();

        for neighbor in neighbor_nodes {
            graph.entry(node.clone()).or_default().insert(neighbor.clone());
            graph.entry(neighbor).or_default().insert(node.clone());
        }
    }
    println!("graph: {:?}", graph);

    "output".to_string()
}
