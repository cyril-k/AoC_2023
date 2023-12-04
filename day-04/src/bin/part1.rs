use std::collections::HashSet;

fn main () {
    let input = include_str!("./part1.txt");
    let output = part1(input);
    println!("{output}")
}

#[derive(Debug)]
struct ScratchCard {
    combination: HashSet<usize>,
    candidate: HashSet<usize>
}

impl ScratchCard {
    fn cash_out(&self) -> u32 {
        let matching_count = self.combination
            .intersection(&self.candidate)
            .count();
        if matching_count > 0 {
            return 2_u32.pow((matching_count - 1) as u32)
        }
        0
    }
}

fn part1(input: &str) -> String {
    let mut card_sum: u32 = 0;
    // parse cards
    for line in input.lines() {
        let splits = line
            .split(": ")
            .last()
            .unwrap()
            .split(" | ")
            .collect::<Vec<&str>>();
        
        let card = ScratchCard {
            combination: process_splits(splits[0]),
            candidate: process_splits(splits[1]),
        };
        card_sum += card.cash_out();
    }
    // "output".to_string()
    card_sum.to_string()
}

fn process_splits(split: &str) -> HashSet<usize> {
    split.split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<HashSet<usize>>()
}


