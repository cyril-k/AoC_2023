use std::collections::HashMap;
use core::ops::Range;

fn main () {
    let input = include_str!("./part1.txt");
    let output = part1(input);
    println!("{output}")
}

#[derive(Debug)]
struct NumberMap {
    input_ranges: Vec<Range<i64>>,
    output_ranges: Vec<Range<i64>>,
}

impl NumberMap {
    fn new(input_start: i64, output_start: i64, seq_len: i64) -> Self {
        Self {
            // name: name.to_string(),
            input_ranges: vec![input_start..input_start + seq_len],
            output_ranges: vec![output_start..output_start + seq_len],
        }
    }

    fn update(&mut self, input_start: i64, output_start: i64, seq_len: i64) {
        self.input_ranges.push(input_start..input_start + seq_len);
        self.output_ranges.push(output_start..output_start + seq_len);
    }

    fn map_number(&self, input: i64) -> i64 {
        for (input_range, output_range) in self.input_ranges.iter().zip(&self.output_ranges) {
            if input_range.contains(&input) {
                let offset = output_range.end - input_range.end;
                return input + offset
            }
        }
        input
    }
}

fn part1(input: &str) -> String {
    let mut nums :Vec<i64> = vec![];
    let mut maps: HashMap<&str, NumberMap> = HashMap::new();
    let mut last_map_name = "";
    let mut map_list: Vec<&str> = vec![];
    input.split("\n").enumerate().for_each(|(i, paragraph)| {
        if i == 0 {
            paragraph
                .split(": ")
                .last()
                .unwrap()
                .split(" ")
                .filter(|s| !s.is_empty())
                .for_each(|s| nums.push(s.parse::<i64>().unwrap()));
        } else {
            for line in paragraph.lines() {
                match line.split(" ").collect::<Vec<&str>>().as_slice() {
                    [map_name, "map:"] => {
                        last_map_name = map_name.to_owned();
                        map_list.push(map_name);
                    },
                    [num1, num2, num3] => {
                        let key_start = num2.parse::<i64>().unwrap();
                        let val_start = num1.parse::<i64>().unwrap();
                        let seq_len = num3.parse::<i64>().unwrap();
                        if !maps.contains_key(last_map_name) {
                            maps
                                .insert(
                                    last_map_name, 
                                    NumberMap::new(key_start, val_start, seq_len)
                                );
                        } else {
                            maps
                                .get_mut(last_map_name)
                                .unwrap()
                                .update(key_start, val_start, seq_len);
                        }
                    },
                    _ => (),
            }
        }
        }
    });

    map_list.reverse();
    let traced_nums = trace_values(map_list, nums, &maps);

    traced_nums
        .unwrap()
        .into_iter()
        .min()
        .unwrap()
        .to_string()
    // "output".to_string()
}

fn trace_values(
    mut map_list: Vec<&str>,
    nums: Vec<i64>,
    maps: &HashMap<&str, NumberMap>,
) -> Option<Vec<i64>> {
    match map_list.pop() {
        Some(map_name) => {
            let nums = nums
                .iter()
                .map(|num| {
                    maps
                        .get(map_name)
                        .unwrap()
                        .map_number(*num)
                    })
                .collect();
            trace_values(map_list, nums, &maps)
        },
        None => Some(nums),
    }
}
