use std::collections::HashSet;


fn main () {
    let input = include_str!("./part1.txt");
    let output = part1(input);
    println!("{output}")
}

fn part1(input: &str) -> String {
    let mut history_sum: isize = 0;

    for line in input.lines() {
        let input_a = line
            .split(" ")
            .filter_map(|s| s.parse::<isize>().ok())
            .collect::<Vec<isize>>();

        history_sum += extrapolate_array(&input_a);
    };
    history_sum.to_string()
    // "output".to_string()
}

fn compute_delta_array(input_a: &Vec<isize>) -> Vec<isize> {
    let mut output_a: Vec<isize> = vec![];
    for (i, element) in input_a.iter().enumerate() {
        let next_elem: isize = match input_a.get(i+1) {
            Some(elem ) => *elem,
            None => break,
        };
        let delta = next_elem - element;
        output_a.push(delta);
    }
    output_a
}

fn is_monotonical(input_a: &Vec<isize>) -> bool {
    input_a
        .iter()
        .collect::<HashSet<_>>()
        .len() == 1
}

fn extrapolate_array(input_a: &Vec<isize>) -> isize {
    let mut result: isize = 0;
    let mut seq = input_a.clone();

    loop {
        result += seq.last().unwrap();
        if is_monotonical(&seq) {
            break
        }
        let delta_seq = compute_delta_array(&seq);
        seq = delta_seq;
    }

    result
}