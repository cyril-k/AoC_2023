use anyhow::Result;

fn main () {
    let input = include_str!("./part1.txt");
    let output = part1(input);
    println!("{output}")
}

#[derive(Debug)]
struct GameSet {
    red: usize,
    green: usize,
    blue: usize,
}

impl GameSet {
    fn update(&mut self, splits: Vec<&str>) {
        let count = match parse_from_splits(&splits) {
            Ok(val) => val,
            Err(err) => panic!("{err}")
        };
        match splits.last().copied() {
            Some("red") => {
                if count > self.red {
                    self.red = count
                }
            },
            Some("green") => {
                if count > self.green {
                    self.green = count
                }
            },
            Some("blue") => {
                if count > self.blue {
                    self.blue = count
                }
            },
            Some(_) => panic!("invalid color"),
            None => panic!("what's going on"),
        }
    }

    fn validate(&self, other: &GameSet) -> bool {
        if self.red <= other.red 
            && self.green <= other.green 
            && self.blue <= other.blue {
            true
        } else { false }
    }
}

fn part1(input: &str) -> String {
    // the reference set
    let ref_game_set = GameSet {
        red: 12,
        green: 13,
        blue: 14,
    };
    // parse sets
    let mut ids_sum: usize = 0;
    // let mut parsed_sets = vec![];
    for (game_i, line) in input.lines().enumerate() {
        let sets = line
            .split(": ")
            .collect::<Vec<&str>>()
            .last()
            .copied()
            .unwrap()
            .split("; ")
            .collect::<Vec<&str>>();
        let mut game_set = GameSet {
            red: 0,
            green: 0,
            blue: 0,
        };
        for set in sets.into_iter() {
            for color in set.split(", ") {
                let splits = color.split(" ").collect::<Vec<&str>>();
                game_set.update(splits);
            }
        }
        println!("game {} : {:?}", game_i+1, game_set);
        if game_set.validate(&ref_game_set) {
            ids_sum += game_i+1;
        }
        // parsed_sets.push(game_set);
    }
    // "output".to_string()
    ids_sum.to_string()
}

fn parse_from_splits(splits: &Vec<&str>) -> Result<usize> {
    splits.first().unwrap().parse::<usize>().map_err(anyhow::Error::from)
}

