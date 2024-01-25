use std::collections::{HashMap, HashSet, VecDeque};
use rand::Rng;

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

    let result = brute_force(graph);
    
    // "output".to_string()
    result.to_string()
}

fn brute_force(mut graph: HashMap<String, HashSet<String>>) -> usize {
    loop {
        let mut visited_edges: HashMap<(String, String), usize> = HashMap::new();
        let mut memory: HashMap<(String, String), Vec<String>> = HashMap::new();
        let mut seed = rand::thread_rng();

        for n1 in graph.keys() {
            for n2 in graph.keys() {
                if n1 == n2 || seed.gen::<f64>() < 0.99999 {
                    continue;
                }
                
                let path = bfs_search(&graph, n1.clone(), n2.clone(), &mut memory);
                for n in path.windows(2) {
                    if visited_edges.contains_key(&(n[0].clone(), n[1].clone())) {
                        visited_edges.entry((n[0].clone(), n[1].clone())).and_modify(|v| *v += 1);
                    } else if visited_edges.contains_key(&(n[1].clone(), n[0].clone())) {
                        visited_edges.entry((n[1].clone(), n[0].clone())).and_modify(|v| *v += 1);
                    } else {
                        visited_edges.insert((n[0].clone(), n[1].clone()), 1);
                    }
                }
            }
        }
        let mut visited_edges = visited_edges.into_iter().collect::<Vec<_>>();
        visited_edges.sort_by(|e1, e2| e2.1.cmp(&e1.1));

        let edges_to_cut: Vec<(String, String)> = visited_edges.into_iter().map(|e| e.0).take(3).collect();

        for e in edges_to_cut {
            graph.entry(e.0.clone()).and_modify(|x| {
                x.remove(&e.1);
            });
            graph.entry(e.1.clone()).and_modify(|x| {
                x.remove(&e.0);
            });
        }

        let mut group_1 = 0;
        let candidate_node = graph.keys().next().unwrap().clone();
        let mut memory: HashMap<(String, String), Vec<String>> = HashMap::new();

        for node in graph.keys() {
            let path = bfs_search(&graph, node.clone(), candidate_node.clone(), &mut memory);
            if !path.is_empty() {
                group_1 += 1;
            }
        }

        let group_2 = graph.keys().count() - group_1;

        let result = group_1 * group_2;

        if result > 0 {
            return result;
        }

    }
}


fn bfs_search(
    graph: &HashMap<String, HashSet<String>>, 
    start: String, 
    finish: String,
    memory: &mut HashMap<(String, String), Vec<String>>
) -> Vec<String> {
    if let Some(path) = memory.get(&(start.clone(), finish.clone())) {
        return path.clone();
    }

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut predecessors = HashMap::new();

    queue.push_back(start.clone());
    visited.insert(start.clone());

    while let Some(node) = queue.pop_front() {
        if node == *finish {
            return reconstruct_path(start.clone(), finish.clone(), &predecessors, memory);
        }

        if let Some(neighbors) = graph.get(&node) {
            for neighbor in neighbors {
                if visited.insert(neighbor.clone()) {
                    queue.push_back(neighbor.clone());
                    predecessors.insert(neighbor.clone(), node.clone());
                }
            }
        }
    }

    Vec::new()
}

fn reconstruct_path(
    start: String,
    finish: String,
    predecessors: &HashMap<String, String>,
    memory: &mut HashMap<(String, String), Vec<String>>,
) -> Vec<String> {
    let mut path = Vec::new();
    let mut current = finish.clone();

    while current != *start {
        path.push(current.clone());
        current = predecessors[&current].clone();
    }
    path.push(start.clone());
    path.reverse();

    memory.insert((start.clone(), finish.clone()), path.clone());
    memory.insert((finish.clone(), start.clone()), path.clone());

    path
}
