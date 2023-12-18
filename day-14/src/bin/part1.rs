use std::collections::HashMap;

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

    let _array = lines
        .iter()
        .flat_map(|line| {
            line.chars().map(move |c| {
                new_position(c)
            })
        })
        .collect::<Vec<isize>>();
    let array = Array2::from_shape_vec((rows, cols), _array)
    .unwrap();

    //"output".to_string()
    find_sum(array).to_string()
}

fn new_position(c: char) -> isize {
    match c {
        'O' => 1,
        '#' => -1,
        _ => 0,
    }
}

fn find_sum(array: Array2<isize>) -> usize {
    let mut sum: usize = 0;
    for col in array.columns() {
        let mut counter: HashMap<usize, usize> = HashMap::new();
        let mut curr_key: usize = col.len();
        for (i, elem) in col.iter().enumerate() {
            if *elem == 1 {
                *counter.entry(curr_key).or_insert(0) += 1;
            } else if *elem == -1 {
                curr_key = col.len() - (i + 1);
            }
        };
        sum += counter
            .into_iter()
            .map(|(k, v)| sum_sequence(k, v) )
            .sum::<usize>();
    };

    sum
}

fn sum_sequence(m: usize, n:usize) -> usize {
    (n/2) * (2*m - n + 1) + (n%2) * (m - n/2)
}



