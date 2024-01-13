use std::collections::HashSet;
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

    let array = Array2::from_shape_fn((rows, cols), |(y, x)| {
        lines
            .get(y)
            .and_then(|line| line.chars().nth(x))
            .unwrap()
    });

    let start_col = find_col(&array.row(0));
    let end_col = find_col(&array.row(array.ncols() - 1));

    let paths = find_all_paths((0, start_col), ((array.ncols() - 1), end_col), &array);

    (paths.iter().map(|p| p.len()).max().unwrap() - 1).to_string() // subtract 1 for initial step

    // "output".to_string()
}

fn find_col(row: &ndarray::ArrayView1<char>) -> usize {
    row
        .iter()
        .position(|&c| c == '.')
        .unwrap()
}

fn get_neighbors(pos: &(usize, usize), array: &Array2<char>) -> Vec<(usize, usize)> {
    let mut neighbours = Vec::new();
    let directions: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)]; // u, d, l, r

    for dir in directions {
        let new_pos = (
            pos.0.wrapping_add_signed(dir.0), 
            pos.1.wrapping_add_signed(dir.1)
        );
        if let Some(tile) = array.get(new_pos) {
            match tile {
                '.' => neighbours.push(new_pos),
                'v' if dir != (-1, 0) => neighbours.push(new_pos),
                '^' if dir != (1, 0) => neighbours.push(new_pos),
                '>' if dir != (0, -1) => neighbours.push(new_pos),
                '<' if dir != (0, 1) => neighbours.push(new_pos),
                _ => (),
            }
        }
    }

    neighbours
}

fn dfs_search(
    current: (usize, usize), 
    target: (usize, usize), 
    array: &Array2<char>, 
    visited: &mut HashSet<(usize, usize)>, 
    current_path: &mut Vec<(usize, usize)>, 
    all_paths: &mut Vec<Vec<(usize, usize)>>
) {

    if visited.contains(&current) {
        return;
    }

    current_path.push(current.clone());
    visited.insert(current.clone());

    if current == target {
        all_paths.push(current_path.clone());
    } else {
        for neighbor_pos in get_neighbors(&current, array) {
            dfs_search(neighbor_pos, target, array, visited, current_path, all_paths);
        }
    }

    current_path.pop();
    visited.remove(&current);
}

fn find_all_paths(start: (usize, usize), target: (usize, usize), array: &Array2<char>) -> Vec<Vec<(usize, usize)>> {
    let mut visited = HashSet::new();
    let mut all_paths = Vec::new();
    let mut current_path = Vec::new();

    dfs_search(start, target, array, &mut visited, &mut current_path, &mut all_paths);

    all_paths
}