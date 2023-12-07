
fn main () {
    let input = include_str!("./part1.txt");
    let output = part1(input);
    println!("{output}")
}

#[derive(Debug)]
struct BoatRace {
    time: usize,
    best_distance: usize,
}

impl BoatRace {
    fn count_wins(&self) -> usize {
        let mut outcome_count: usize = 0;
        for time in 1..self.time {
            // println!("time {}", time);
            let traveled = time * (self.time - time);
            // println!("traveled {}", traveled);
            if traveled > self.best_distance {
                outcome_count += 1;
            }
        }
        outcome_count
    }
}

fn part1(input: &str) -> String {
    let mut wins_product: usize = 1;
    let mut times: Vec<usize> = vec![];
    let mut distances: Vec<usize> = vec![];

    for line in input.lines() {
        match line.split(" ").filter(|s| !s.is_empty()).collect::<Vec<&str>>().as_slice() {
            ["Time:", nums @ ..] => times.extend(nums.into_iter().map(|s| s.parse::<usize>().unwrap())),
            ["Distance:", nums @ ..] => distances.extend(nums.into_iter().map(|s| s.parse::<usize>().unwrap())),
            _ => (),
        }
    }
    // println!("times {:?}", times);
    // println!("distances {:?}", distances);
    let races = times.into_iter().zip(distances).map(|(time, best_distance)| {
        BoatRace {
            time,
            best_distance,
        }
    }).collect::<Vec<BoatRace>>();

    for race in races {
        // println!("wins {}", race.count_wins());
        wins_product *= race.count_wins();
    };

    // "output".to_string()
    wins_product.to_string()
}



