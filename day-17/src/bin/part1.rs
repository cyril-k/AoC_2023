use std::{collections::{HashMap, BinaryHeap}, os::linux::raw};
use ndarray::{self, Array2};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

fn main () {
    let input = include_str!("./part1.txt");
    let output = part1(input);
    println!("{output}")
}

fn part1(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let rows = lines.len();
    let cols = lines[0].chars().count();

    let array = Array2::from_shape_vec(
        (rows, cols), 
        lines
            .iter()
            .flat_map(|line| line
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize))
            .collect())
        .expect("Failed to reshape into 2D array");

    let graph = graph_from_array(array);
    
    let start = Node::new((0, 0));
    let distances = dijkstra(&graph, start, 3);

    println!("{:?}", distances);

    "output".to_string()
    // sum.to_string()

}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
enum Position {
    Up,
    Down,
    Left,
    Right,
    None,
}

type Index = (usize, usize);

#[derive(Clone, Copy, Debug)]
struct Node {
    index: Index,
    pos: Position,
    consecutive: usize,
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

impl Eq for Node {}

impl Node {
    fn new(index: Index) -> Self {
        Node { index, pos: Position::None, consecutive: 0 }
    }

    fn with_direction(index: Index, pos: Position, consecutive: usize) -> Self {
        Node { index, pos, consecutive }
    }

    fn pos(&self, other: &Node) -> Position {
        if self.index.0 > other.index.0 {
            Position::Up
        } else if self.index.0 < other.index.0 {
            Position::Down
        } else if self.index.1 > other.index.1 {
            Position::Left
        } else if self.index.1 < other.index.1 {
            Position::Right
        } else {
            Position::None
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Edge {
    node: Node,
    cost: usize,
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type Graph = HashMap<Node, Vec<Edge>>;


fn graph_from_array(array: Array2<usize>) -> Graph {
    let mut graph = Graph::new();
    let (rows, cols) = array.dim();

    array
        .indexed_iter()
        .for_each(|((row, col), _ )| {
            let mut edges = Vec::new();

            if col > 0 { // Left
                update_edges(row, col - 1, &mut edges, &array);
            }
            if col < rows - 1 { // Right
                update_edges(row, col + 1, &mut edges, &array);
            }
            if row > 0 { // Down
                update_edges(row - 1, col, &mut edges, &array);
            }
            if row < cols - 1 { // Up
                update_edges(row + 1, col, &mut edges, &array);
            }

            graph.insert(Node::new((row, col)), edges);
        });
    graph
}

fn update_edges(
    row: usize, 
    col: usize, 
    edges: &mut Vec<Edge>, 
    array: &Array2<usize>) {
    
    let index: Index = (row, col);
    edges.push(Edge { node: Node::new(index), cost: *array.get(index).unwrap() });
}

fn dijkstra(graph: &Graph, start: Node, max_consecutive: usize) -> HashMap<Node, usize> {
    let mut distances: HashMap<Node, usize> = HashMap::new();
    let mut heap = BinaryHeap::new();

    *distances.entry(start).or_insert(usize::MAX) = 0;
    heap.push(Edge { node: start, cost: 0 });

    while let Some(Edge { node, cost }) = heap.pop() {
        if cost > *distances.entry(node).or_insert(usize::MAX) {
            continue;
        }

        for edge in graph.get(&node).unwrap() {
            let new_consecutive = if edge.node.pos == node.pos {
                node.consecutive + 1
            } else {
                1
            };

            println!("node {:?}, consecutive count {}", edge.node, new_consecutive);

            if new_consecutive <= max_consecutive {
                // continue; // Skip this edge if it exceeds the consecutive move limit
                let next_node = Node::with_direction(edge.node.index, edge.node.pos, new_consecutive);
                let next = Edge { node: next_node, cost: cost + edge.cost };
    
                if next.cost < *distances.entry(next.node).or_insert(usize::MAX) {
                    *distances.entry(next.node).or_insert(usize::MAX) = next.cost;
                    heap.push(next);
                }
            }

        }
    }

    distances
}

// fn dijkstra(graph: &Graph, start: Node) -> HashMap<Node, usize> {
//     let mut distances: HashMap<Node, usize> = HashMap::new();
//     let mut heap = BinaryHeap::new();

//     *distances.entry(start).or_insert(usize::MAX) = 0;
//     heap.push(Edge { node: start, cost: 0 });

//     // let mut dir_counter: HashMap<Position, usize> = HashMap::new();
//     // let mut dir_map: HashMap<Node, Vec<Position>> = HashMap::new();
//     // Vec<Position> = Vec::new();

//     while let Some(Edge {node, cost}) = heap.pop() {
//         println!("current node {:?}", node);
//         if cost > *distances.entry(node).or_insert(usize::MAX) {
//             continue;
//         };


//         for edge in graph.get(&node).unwrap() {
//             // let dir = node.pos(&edge.node);
//             // let dir_counter = dir_map.entry(edge.node).or_insert(vec![]);
//             // // *dir_counter.entry(dir).or_insert(0) += 1;
//             // if dir_counter.contains(&dir) {
//             //     dir_counter.push(dir)
//             // } else {
//             //     *dir_counter = vec![dir, ] // reset direction counter
//             // }
//             // println!("{:?}", dir_counter); 
            
//             // if dir_counter.len() <= 3 {
//                 let next = Edge { node: edge.node, cost: cost + edge.cost};
    
//                 if next.cost < *distances.entry(next.node).or_insert(usize::MAX) {
//                     *distances.entry(next.node).or_insert(usize::MAX) = next.cost;
//                     heap.push(next);
//                 }

//             // }

//         }
//     }

//     distances
// }