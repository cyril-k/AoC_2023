use std::collections::HashSet;

fn main () {
    let input = include_str!("./part1.txt");
    let output = part2(input);
    println!("{output}")
}

#[derive(Debug)]
struct ScratchCard {
    combination: HashSet<usize>,
    candidate_combination: HashSet<usize>
}

impl ScratchCard {
    fn get_count(&self) -> usize {
        self.combination
            .intersection(&self.candidate_combination)
            .count()
    }
}

fn part2(input: &str) -> String {
    let mut cards_count_array: Vec<usize> = vec![1; input.lines().count()];
    
    // parse cards
    for (card_i, line) in input.lines().enumerate() {
        let splits = line
            .split(": ")
            .last()
            .unwrap()
            .split(" | ")
            .collect::<Vec<&str>>();
        
        let card = ScratchCard {
            combination: process_splits(splits[0]),
            candidate_combination: process_splits(splits[1]),
        };

        ((card_i + 1)..(card_i + 1 + card.get_count()))
            .into_iter()
            .for_each(|copied_card| {
                cards_count_array[copied_card] += cards_count_array[card_i]
            });
    }
    cards_count_array.iter().sum::<usize>().to_string()
}

fn process_splits(split: &str) -> HashSet<usize> {
    split.split(" ")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<HashSet<usize>>()
}



