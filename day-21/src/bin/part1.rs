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

    let maze_array = Array2::from_shape_fn((rows, cols), |(y, x)| {
        lines
            .get(y)
            .and_then(|line| line.chars().nth(x))
            .unwrap()
    });

    let start = maze_array.indexed_iter().find_map(|(index, c)| {
        if *c == 'S' {
            Some(index)
        } else { None }
    }).unwrap();

    (find_positions(&maze_array, start, 64).len() + 1).to_string() // add starting pos

    // "output".to_string()
}

fn find_positions(array:&Array2<char>, start: (usize, usize), target_step: usize) -> HashSet<(usize, usize)> {
    let mut positions = HashSet::new();
    let directions: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)]; // u, d, l, r

    positions.insert(start);

    for _ in 0..target_step  {
        let mut new_positions = HashSet::new();
        for position in positions {
            for (dy, dx) in directions.iter() {
                let new_position = (
                    position.0.wrapping_add_signed(*dy), 
                    position.1.wrapping_add_signed(*dx)
                );

                if array.get(new_position).map_or(false, |&tile| tile == '.') {
                    new_positions.insert(new_position);
                }
            }
        }
        positions = new_positions;
    }
    positions
}
