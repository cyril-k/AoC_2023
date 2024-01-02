use std::collections::HashMap;

use regex::Regex;

fn main () {
    let input = include_str!("./part1.txt");
    let output = part1(input);
    println!("{output}")
}

fn part1(input: &str) -> String {
    let re = Regex::new(r"([a-z]+)\{(.+)\}").unwrap();
    let splits: Vec<_> = input.split("\n\n").collect();
    let mut stages = HashMap::new();

    let re_col = Regex::new(r"\{(.+)\}").unwrap();
    let mut collections = Vec::new();
    
    // Recover stages
    for line in splits[0].lines() {
        if let Some(caps) = re.captures(line) {
            let stage_id = caps.get(1).map_or("", |m| m.as_str());
            let conditions = caps.get(2).map_or("", |m| m.as_str());
            stages.insert(stage_id, Vec::new());

            conditions
                .split(',')
                .for_each(|c| stages
                    .get_mut(stage_id)
                    .unwrap()
                    .push(Condition::new(c))
                );
        }
    };

    // Recover pieces
    for line in splits[1].lines () {
        if let Some(caps) = re_col.captures(line) {
            collections.push(new_collection(&caps[1]));
        }
    };

    let score = collections.iter().map(|c| rate(c, &stages)).sum::<usize>();

    score.to_string()
    // "output".to_string()
}

fn rate(col: &Collection, stages: &HashMap<&str, Vec<Condition>>) -> usize {
    let mut curr_stage = String::from("in");
    while curr_stage != String::from("A") {
        if let Some(stage_seq) = stages.get(curr_stage.as_str()) {
            for cond in stage_seq {
                if let Some (stage) = cond.check_cond(col) {
                    if stage == String::from("R") {
                        return 0
                    }
                    curr_stage = stage;
                    break
                }
            }
        }
    }

    col.values().sum::<usize>()
}

type Collection = HashMap<PartType, usize>;

fn new_collection(text: &str) -> Collection {
    let mut collection = Collection::new();

    text.split(',')
        .for_each(|attr| {
            let splits: Vec<&str> = attr.split('=').collect();
            let piece = PartType::new(splits[0].chars().next().unwrap());
            let val = splits[1].parse::<usize>().unwrap();
            collection.insert(piece, val);
        });
    
    collection
}


#[derive(Debug, Hash, Eq, PartialEq)]
enum PartType {
    X,
    M,
    A,
    S,
}

impl PartType {
    fn new(c: char) -> Self {
        match c {
            'x' => PartType::X,
            'm' => PartType::M,
            'a' => PartType::A,
            's' => PartType::S,
            _ => panic!("invalid") }
    }
}

#[derive(Debug)]
struct Condition {
    part_type: Option<PartType>,
    condition: Option<char>,
    value: Option<usize>,
    next_step: String
}

impl Condition {
    fn new(text: &str) -> Self {
        let mut part_type = None;
        let mut condition = None;
        let mut value = None;
        let mut next_step = String::new();

        let re = Regex::new(r"([a-z]+)([<>])(\d+):([A-Za-z]+)").unwrap();
        match re.captures(text) {
            Some(caps) => {
                part_type = caps.get(1).and_then(|m| Some(PartType::new(m.as_str().chars().next()?)));
                condition = caps.get(2).and_then(|m| m.as_str().chars().next());
                value = caps.get(3).and_then(|m| m.as_str().parse::<usize>().ok());
                next_step = caps.get(4).map_or(next_step, |m| m.as_str().to_string());
            }
            None => {
                next_step = text.to_owned();
                }
        };
        
        Condition { part_type, condition, value, next_step }
    }

    fn check_cond(&self, col: &Collection) -> Option<String> {
        match self.condition {
            Some('<') => if col.get(self.part_type.as_ref()?)? < &self.value? {
                return Some(self.next_step.clone())
            },
            Some('>') => if col.get(self.part_type.as_ref()?)? > &self.value? {
                return Some(self.next_step.clone())
            }
            None => { return Some(self.next_step.clone()) },
            _ => unreachable!(),
        }
        
        None
    }
}
