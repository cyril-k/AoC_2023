use ndarray::{self, Array2};

fn main () {
    let input = include_str!("./part1.txt");
    let output = part1(input);
    println!("{output}")
}

fn part1(input: &str) -> String {


    "output".to_string()
}

fn get_neighbors(pos: &(usize, usize), array: &Array2<char>) -> Vec<(usize, usize)> {
    let mut neighbours = Vec::new();
    let directions: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)]; // u, d, l, r

    for dir in directions {
        let new_pos = (pos.0.wrapping_add_signed(dir.0), pos.1.wrapping_add_signed(dir.1));
        if let Some(tile) = array.get(new_pos) {
            match tile {
                '.' => neighbours.push(new_pos.clone()),
                'v' => if dir != (-1, 0) {
                    neighbours.push(new_pos.clone());
                },
                '^' => if dir != (1, 0) {
                    neighbours.push(new_pos.clone());
                },
                '>' => if dir != (0, -1) {
                    neighbours.push(new_pos.clone());
                },
                '<' => if dir != (0, 1) {
                    neighbours.push(new_pos.clone());
                },
                _ => (),
            }
        }
    }

    neighbours
}