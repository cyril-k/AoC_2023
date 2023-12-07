use std::collections::HashMap;
use std::cmp::Ordering;

fn main () {
    let input = include_str!("./part1.txt");
    let output = part1(input);
    println!("{output}")
}

fn part1(input: &str) -> String {
    let mut hands: Vec<Hand> = input
        .lines()
        .map(|line| {
            let splits: Vec<&str> = line.split(" ").collect();
            let hand = Hand::new(
                splits[0], 
                splits[1].parse().unwrap()
            );
            hand
        })
        .collect();

    quick_sort(&mut hands);

    let wins: usize = hands
        .iter()
        .enumerate()
        .map(|(rank, hand)| hand.bid * (rank + 1))
        .sum();

    // "output".to_string()
    wins.to_string()
}

fn quick_sort(array: &mut Vec<Hand>) {
    let len = array.len();
    _quick_sort(array, 0, (len as i64) - 1);
}

fn _quick_sort(array: &mut Vec<Hand>, low: i64, high: i64) {
    if low < high {
        let p = partition(array, low, high);
        _quick_sort(array, low, p - 1);
        _quick_sort(array, p + 1, high);
    }
}

fn partition(array: &mut Vec<Hand>, low: i64, high: i64) -> i64 {
    let pivot = high as usize;
    let mut left = low - 1;
    let mut right = high;

    loop {
        left += 1;
        while left < high && array[left as usize] < array[pivot] {
            left += 1;
        }

        right -= 1;
        while right > 0 && array[right as usize] > array[pivot] {
            right -= 1;
        }

        if left >= right {
            break;
        } else {
            array.swap(left as usize, right as usize);
        }
    }
    array.swap(left as usize, pivot as usize);
    left
}

fn score_card(card: &char) -> usize {
    match card.to_digit(10).ok_or(card) {
        Ok(score) => score as usize,
        Err(card) => match card {
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!("invalid card type: {card}")    
        }
    }
}

#[derive(Debug, PartialEq)]
struct Hand {
    hand_type: HandType,
    hand_value: String,
    bid: usize,
}

#[derive(Debug, PartialEq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Hand {
    fn new(hand_value: &str, bid: usize) -> Self {
        let card_counts: HashMap<char, usize> =
        hand_value
            .to_lowercase()
            .chars()
            .fold(HashMap::new(), |mut map, c| {
                *map.entry(c).or_insert(0) += 1;
                map
            });

        let hand_type = match card_counts.len() {
            1 => Some(HandType::FiveOfAKind),
            2 => match card_counts.values().max() {
                    Some(4) => Some(HandType::FourOfAKind),
                    Some(3) => Some(HandType::FullHouse),
                    _ => None,
                },
            3 => match card_counts.values().max() {
                Some(3) => Some(HandType::ThreeOfAKind),
                Some(2) => Some(HandType::TwoPair),
                _ => None,
                },
            4 => Some(HandType::OnePair),
            _ => Some(HandType::HighCard),
        };

        Self { 
            hand_type: hand_type.unwrap(), 
            hand_value: hand_value.to_owned(),
            bid
        }
    }

    fn rank(&self) -> usize {
        match self.hand_type {
            HandType::FiveOfAKind => 7,
            HandType::FourOfAKind => 6,
            HandType::FullHouse => 5,
            HandType::ThreeOfAKind => 4,
            HandType::TwoPair => 3,
            HandType::OnePair => 2,
            HandType::HighCard => 1,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Some(self.cmp(other))
        if self.rank() > other.rank() {
            Some(Ordering::Greater)
        } else if self.rank() < other.rank() {
            Some(Ordering::Less)
        } else {
            let card_comparison = self.hand_value
                .chars()
                .zip(other.hand_value.chars())
                .find_map(|(this_c, other_c)| {
                    match score_card(&this_c).cmp(&score_card(&other_c)) {
                        Ordering::Greater => Some(Ordering::Greater),
                        Ordering::Less => Some(Ordering::Less),
                        _ => None,
                    }
                });
            Some(card_comparison.unwrap_or(Ordering::Equal))
        }
    }
}
