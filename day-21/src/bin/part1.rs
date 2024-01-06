use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Ordering;
use std::cmp::Reverse;

use ndarray::{self, Array2};

const MAX_COST: usize = 10000;

fn main () {
    let input = include_str!("./part1.txt");
    let output = part1(input);
    println!("{output}")
}

fn part1(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let rows = lines.len();
    let cols = lines[0].chars().count();

    let maze_array = Array2::from_shape_fn((rows, cols), |(y, x)| {
        lines
            .get(y)
            .and_then(|line| line.chars().nth(x))
            .unwrap()
    });

    println!("{:?}", maze_array);

    let res = a_star_search(Node::new((5, 5)), Node::new((0, 0)), &maze_array);
    println!("{:?}", res);

    "output".to_string()
}


#[derive(Eq, PartialEq, Debug, Clone)]
struct Node {
    position: (usize, usize),
    g_cost: usize,
    h_cost: usize,
}

impl Node {
    fn new(position: (usize, usize)) -> Self {
        Node { position, g_cost: MAX_COST, h_cost: 0 }
    }

    fn f_cost(&self) -> usize {
        self.g_cost + self.h_cost
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_cost().cmp(&self.f_cost()).reverse()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn manhattan_distance(start: (usize, usize), goal: (usize, usize)) -> usize {
    ((start.0 as isize - goal.0 as isize).abs() + (start.1 as isize - goal.1 as isize).abs()) as usize
}

fn get_neighbors(node: &Node, maze: &Array2<char>) -> Vec<Node> {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // r, d, l, u
    directions.iter().filter_map(|&(dx, dy)| {
        let new_pos = ((node.position.0 as isize + dx) as usize, (node.position.1 as isize + dy) as usize);
        maze.get(new_pos).and_then(|&cell| {
            if cell == '.' || cell == 'S' { Some(Node::new(new_pos)) } else { None }
        })
    }).collect()
}

fn reconstruct_path(came_from: HashMap<(usize, usize), (usize, usize)>, mut current_pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut path = vec![current_pos];
    println!("came_from: {:?}", came_from);
    while let Some(parent_pos) = came_from.get(&current_pos) {
        println!("parent_pos: {:?}", parent_pos);
        path.push(*parent_pos);
        current_pos = *parent_pos;
        break
    }
    path.reverse();
    path
}

fn a_star_search(start: Node, goal: Node, maze: &Array2<char>) -> Option<Vec<(usize, usize)>> {
    let mut open_set = BinaryHeap::new();
    let mut open_set_tracker = HashSet::new();
    let mut came_from = HashMap::new();

    open_set_tracker.insert(start.position);
    open_set.push(Reverse(start));

    let mut counter: usize = 0;

    while let Some(Reverse(current)) = open_set.pop() {
        println!("current node: {:?}", current);
        // println!("goal node: {:?}", goal);
        open_set_tracker.remove(&current.position);
        if current.position == goal.position {
            return Some(reconstruct_path(came_from, current.position));
        }

        counter += 1;
        if counter > 6 {
            break;
        }

        for mut neighbor in get_neighbors(&current, maze) {
            println!("current neighbor: {:?}", neighbor);
            let tentative_g_score = current.g_cost - 1;
            println!("tentative_g_score < neighbor.g_cost: {:?}", tentative_g_score < neighbor.g_cost);
            if tentative_g_score < neighbor.g_cost {
                came_from.insert(neighbor.position, current.position);
                neighbor.g_cost = tentative_g_score;
                neighbor.h_cost = manhattan_distance(neighbor.position, goal.position);
                if !open_set_tracker.contains(&neighbor.position) {
                    open_set_tracker.insert(neighbor.position);
                    open_set.push(Reverse(neighbor));
                }
            }
        }
    }
    println!("open_set_tracker: {:?}", open_set_tracker);
    None
}