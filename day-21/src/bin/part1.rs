use std::collections::{HashSet, VecDeque};
use ndarray::{self, Array2};


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

    // println!("{:?}", maze_array);
    let start = maze_array.indexed_iter().find_map(|(index, c)| {
        if *c == 'S' {
            Some(Node::new(index))
        } else { None }
    }).unwrap();

    let res = bfs_search(start, 64, &maze_array);
    println!("{:?}", res);

    "output".to_string()
}

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
struct Node {
    position: (usize, usize)
}

impl Node {
    fn new(position: (usize, usize)) -> Self {
        Node { position }
    }
}

fn get_neighbors(position: &(usize, usize), maze: &Array2<char>) -> Vec<(usize, usize)> {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // r, d, l, u
    directions.iter().filter_map(|&(dx, dy)| {
        let new_pos = ((position.0 as isize + dx) as usize, (position.1 as isize + dy) as usize);
        maze.get(new_pos).and_then(|&cell| {
            if cell == '.' || cell == 'S' { Some(new_pos) } else { None }
        })
    }).collect()
}

fn bfs_search(start: Node, target_step: usize, maze: &Array2<char>) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back((start, 0));

    let mut current_tracker = HashSet::new();

    while let Some((current, step)) = queue.pop_front() {

        if step == target_step {
            current_tracker.insert(current);
            continue;
        } else if step > target_step {
            break;
        }

        for neighbor_pos in get_neighbors(&current.position, maze) {
                queue.push_back((Node::new(neighbor_pos), step + 1));
        }
    }

    let mut viz_maze = maze.clone();

    for node in current_tracker.iter() {
        *viz_maze.get_mut(node.position).unwrap() = 'O';
    }
    println!("viz_maze:\n {}", viz_maze);

    current_tracker.len()
}
