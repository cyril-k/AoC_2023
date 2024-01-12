use std::collections::{HashMap, BinaryHeap};
use ndarray::{self, Array2};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

use std::cmp::Reverse;

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
        
    let start = (0, 0);
    let goal = (array.nrows()-1, array.ncols()-1);
    let cost = a_star(&array, start, goal, 3);

    println!("{:?}", cost);

    "output".to_string()
    // sum.to_string()

}

#[derive(Clone, Copy, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None, 
}

#[derive(Clone, Copy, Eq, PartialEq)]
struct State {
    position: (usize, usize),
    direction: Direction,
    consecutive: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.position.cmp(&other.position)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn neighbors(current: State, rows: usize, cols: usize, max_consecutive: usize) -> Vec<State> {
    let mut result = Vec::new();
    let (row, col) = current.position;

    let directions = [
        ((row.wrapping_sub(1), col), Direction::Up),
        ((row + 1, col), Direction::Down),
        ((row, col.wrapping_sub(1)), Direction::Left),
        ((row, col + 1), Direction::Right),
    ];

    for &(new_pos, new_dir) in &directions {
        if new_pos.0 < rows && new_pos.1 < cols {
            if new_dir != current.direction && current.consecutive < max_consecutive {
                result.push(State {
                    position: new_pos,
                    direction: new_dir,
                    consecutive: 1,
                });
            } else if new_dir == current.direction {
                result.push(State {
                    position: new_pos,
                    direction: new_dir,
                    consecutive: current.consecutive + 1,
                });
            }
        }
    }

    result
}

fn manhattan_d(node: &(usize, usize), goal: &(usize, usize)) -> usize {
    let dx = (node.0 as isize - goal.0 as isize).abs();
    let dy = (node.1 as isize - goal.1 as isize).abs();
    (dx + dy) as usize
}

fn a_star(array: &Array2<usize>, start: (usize, usize), goal: (usize, usize), max_consecutive: usize) -> usize {
    let mut distances: HashMap<(usize, usize), usize> = HashMap::new();
    let mut heap = BinaryHeap::new();
    let mut came_from: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    distances.insert(start, 0);

    let start_state = State {
        position: start,
        direction: Direction::None,
        consecutive: 0,
    };

    heap.push(Reverse((0 + manhattan_d(&start, &goal), start_state)));
    
    while let Some(Reverse((_, current))) = heap.pop() {
        if current.position == goal {
            break;
        }

        for neighbor in neighbors(current, array.nrows(), array.ncols(), max_consecutive) {
            let next_cost = distances[&current.position] + array.get(neighbor.position).unwrap();

            if next_cost < *distances.entry(neighbor.position).or_insert(usize::MAX) {
                distances.insert(neighbor.position, next_cost);
                came_from.insert(neighbor.position, current.position);
                let total_cost = next_cost + manhattan_d(&neighbor.position, &goal);
                heap.push(Reverse((total_cost, neighbor)));
            }
        }

        
    }
    println!("distances {:?}", distances);
    // distances
    calculate_total_cost(array, &distances, &came_from, start, goal)
}

fn calculate_total_cost(
    array: &Array2<usize>,
    distances: &HashMap<(usize, usize), usize>,
    came_from: &HashMap<(usize, usize), (usize, usize)>,
    start: (usize, usize),
    goal: (usize, usize),
) -> usize {
    let mut total_cost = 0;
    let mut current = goal;
    println!("goal {:?}", goal);
    while let Some(&previous) = came_from.get(&current) {
        total_cost += array.get(current).unwrap_or(&0);
        // println!("total_cost {}", total_cost);
        if previous == start {
            break;
        }
        current = previous;
    }

    total_cost - array.get(start).unwrap()
}