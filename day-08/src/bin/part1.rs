use std::{collections::{HashMap, BTreeMap}, vec};
use regex::Regex;

fn main () {
    let input = include_str!("./part1.txt");
    let output = part1(input);
    println!("{output}")
}

fn part1(input: &str) -> String {
    let re = Regex::new(r"[A-Z]{3}").unwrap();
    let mut steps = "";
    let mut maze: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    input.split("\n").enumerate().for_each(|(i, paragraph)| {
        if i == 0 {
            steps = paragraph;
        } else {
            for line in paragraph.lines() {
                let matches: Vec<&str> = re.find_iter(line).map(|m| m.as_str()).collect();
                maze.insert(matches[0], vec![matches[1], matches[2]]);
            }
        }
    });

    let counter = trace_maze(&maze, steps);

    counter.to_string()
    // "output".to_string()

}

fn trace_maze(maze: &BTreeMap<&str, Vec<&str>>, steps: &str) -> usize {

    // let (current_step, _) = maze.first_key_value().unwrap();
    let current_step = "AAA";
    _trace_maze(maze, steps, current_step)
}

fn _trace_maze<'a>(maze: &'a BTreeMap<&str,  Vec<&str>>, steps: &str,  mut current_step:&'a str) -> usize {
    let mut counter: usize = 0;
    for c in steps.chars().cycle() {
        let this_opt = maze.get(current_step).expect("current step not found in maze");
        current_step = match c {
            'L' => this_opt[0],
            'R' => this_opt[1],
            _ => panic!("invalid char: {}", c),
        };
        counter += 1;
        if current_step == "ZZZ" {
            break;
        }
    }

    counter
}