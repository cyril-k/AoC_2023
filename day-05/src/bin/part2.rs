use std::collections::HashMap;
use core::ops::Range;
use std::cmp::{min, max};

fn main () {
    let input = include_str!("./part2.txt");
    let output = part1(input);
    println!("{output}")
}

#[derive(Debug)]
struct NumberMap {
    // name: String,
    input_ranges: Vec<Range<i64>>,
    output_ranges: Vec<Range<i64>>,
}

impl NumberMap {
    fn new(input_start: i64, output_start: i64, seq_len: i64) -> Self {
        Self {
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
    fn map_ranges(&self, test_range:&Range<i64>) -> Vec<Range<i64>> {
        println!("input range");
        println!("{:?}", test_range);
        // dbg!(test_range.clone());
        let mut new_ranges: Vec<Range<i64>> = vec![]; 
        for (input_range, output_range) in self.input_ranges.iter().zip(&self.output_ranges) {
            if they_intersect(test_range, input_range) {
                let offset = output_range.end - input_range.end;
                println!("offset {}", offset);
                // return input + offset
                let new_range = min(test_range.start, input_range.start) + offset..max(test_range.end, input_range.end) + offset;
                new_ranges.push(new_range.clone());
                println!("the new ranges");
                println!("{:?}", new_ranges);

                if test_range.start < min(test_range.start, input_range.start) {
                    let remainder_before = test_range.start..min(test_range.start, input_range.start);
                    new_ranges.push(remainder_before.clone());
                } else if test_range.end > max(test_range.end, input_range.end) {
                    let remainder_after = max(test_range.end, input_range.end)..test_range.end;
                    new_ranges.push(remainder_after.clone());
                }; 
                // println!("the new ranges");
                // println!("{:?}", new_ranges);
                return new_ranges
             
            }
        }
        new_ranges.push(test_range.clone());
        new_ranges
    }
}

enum IndexType {
    Odd,
    Even,
}

fn extract_indices(vec: &Vec<i64>, index_type: IndexType) -> Vec<i64> {
    vec.iter()
        .enumerate()
        .filter(|(index, _)| match index_type {
            IndexType::Odd => index % 2 != 0,
            IndexType::Even => index % 2 == 0,
        })
        .map(|(_, val)| *val)
        .collect::<Vec<i64>>()
}

fn they_intersect(range1: &Range<i64>, range2: &Range<i64>) -> bool {
    range1.start <= range2.end && range1.end >= range2.start
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

    let num_ranges = extract_indices(&nums, IndexType::Even)
        .iter()
        .zip(&extract_indices(&nums, IndexType::Odd))
        .map(|(range_start, seq_len)| {
            *range_start..*range_start + seq_len
        })
        .collect::<Vec<Range<i64>>>();


    map_list.reverse();
    // let traced_nums = trace_values(map_list, nums, &maps);
    let traced_num_ranges = trace_value_ranges(map_list, num_ranges, &maps);
    dbg!(traced_num_ranges);    
    // traced_nums
    //     .unwrap()
    //     .into_iter()
    //     .min()
    //     .unwrap()
    //     .to_string()
    "output".to_string()
}

fn trace_value_ranges(
    mut map_list: Vec<&str>,
    num_ranges: Vec<Range<i64>>,
    maps: &HashMap<&str, NumberMap>,
) -> Option<Vec<Range<i64>>> {
    match map_list.pop() {
        Some(map_name) => {
            println!("{}", map_name);
            // dbg!(num_ranges.clone());  
            let num_ranges = num_ranges
                .iter()
                .flat_map(|num_range| {
                    maps
                        .get(map_name)
                        .unwrap()
                        .map_ranges(num_range)
                    })
                .collect::<Vec<Range<i64>>>();
            trace_value_ranges(map_list, num_ranges, &maps)
        },
        None => Some(num_ranges),
    }

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
