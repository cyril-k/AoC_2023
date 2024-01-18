use std::collections::{HashMap, BinaryHeap};
use ndarray::{self, Array2};
use std::cmp::Ordering;
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
    let cost = find_path(&array, start, goal, 3);

    // "output".to_string()
    cost.to_string()

}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None, 
}

impl Direction {
    fn is_valid(&self, other: &Direction) -> bool {
        match self {
            Direction::Up if other == &Direction::Down => false,
            Direction::Down if other == &Direction::Up => false,
            Direction::Left if other == &Direction::Right => false,
            Direction::Right if other == &Direction::Left => false,
            _ => true,
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
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
        if new_pos.0 < rows && new_pos.1 < cols && new_dir.is_valid(&current.direction){
            if new_dir != current.direction {
                result.push(State {
                    position: new_pos,
                    direction: new_dir,
                    consecutive: 1,
                });
            } else if new_dir == current.direction && current.consecutive < max_consecutive {
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

fn find_path(array: &Array2<usize>, start: (usize, usize), goal: (usize, usize), max_consecutive: usize) -> usize {
    let mut loss: HashMap<State, usize> = HashMap::new();
    let mut heap: BinaryHeap<Reverse<(usize, State)>> = BinaryHeap::new();
    
    let start_state = State {
        position: start,
        direction: Direction::None,
        consecutive: 0,
    };
    
    loss.insert(start_state, 0);
    heap.push(Reverse((0, start_state)));
    
    while let Some(Reverse((current_cost, current_state))) = heap.pop() {
        if current_state.position == goal {
            return current_cost;
        }
        
        if current_cost > *loss.get(&current_state).unwrap_or(&usize::MAX) {
            continue;
        }
        
        for neighbor in neighbors(current_state, array.nrows(), array.ncols(), max_consecutive) {
            let next_cost = current_cost + array.get(neighbor.position).unwrap();
            if next_cost < *loss.get(&neighbor).unwrap_or(&usize::MAX) {
                loss.insert(neighbor, next_cost);
                heap.push(Reverse((next_cost, neighbor)));
            }
        }
    }
    usize::MAX
}
