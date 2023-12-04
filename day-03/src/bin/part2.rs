use core::ops::Range;
use std::vec;
use std::cmp::max;

fn main() {
    let input = include_str!("./part1.txt");
    let output = part2(input);
    println!("{output}")
}

#[derive(Debug, PartialEq)]
struct PositionalEntry {
    x_coord: Range<usize>,
    y_coord: usize,
}

impl PositionalEntry {
    fn is_gear_ratio(&self, number1: &PositionalEntry, number2: &PositionalEntry) -> bool {
        if number1 == number2 {
            return false
        }
        let y_range = max(0, self.y_coord - 1)..self.y_coord + 2;
        y_range.contains(&number1.y_coord)
            && y_range.contains(&number2.y_coord)
            && are_adjacent(&self.x_coord, &number1.x_coord)
            && are_adjacent(&self.x_coord, &number2.x_coord)
    }
}

fn part2(input: &str) -> String {
    let re_syms = regex::Regex::new(r"(\*)").unwrap();
    let re_nums = regex::Regex::new(r"(\d+)").unwrap();

    let mut symbol_entries = vec![];
    let mut num_entries = vec![];

    let lines: Vec<&str> = input.lines().collect();

    for (i, line) in lines.iter().enumerate() {
        symbol_entries.extend(
            re_syms
                .find_iter(line)
                .map(|m| {
                    PositionalEntry { 
                        x_coord: m.range(), 
                        y_coord: i 
                    }
                }),
        );
        num_entries.extend(
            re_nums
                .find_iter(line)
                .map(|m| {
                    PositionalEntry { 
                        x_coord: m.range(), 
                        y_coord: i 
                    }
                }),
        );
    }
    let cogs = check_cogs(&symbol_entries, &num_entries, &lines);
    // println!("cog ratios: {:?}", cogs);
    cogs.iter().sum::<usize>().to_string()
}
 
fn are_adjacent(a: &Range<usize>, b: &Range<usize>) -> bool {
    a.start <= b.end && a.end >= b.start
}

fn parse_from_lines(lines: &Vec<&str>, pos_entry: &PositionalEntry) -> usize {
    let line = lines[pos_entry.y_coord];
    let num = (line[pos_entry.x_coord.clone()])
        .parse::<usize>()
        .unwrap();
    num
}

fn check_cogs(symbols: &Vec<PositionalEntry>, numbers: &Vec<PositionalEntry>, lines: &Vec<&str>) -> Vec<usize> {
        numbers
            .iter()
            .enumerate()
            .flat_map(|(i, number_entry1)| {
                numbers[i + 1..]
                    .iter()
                    .flat_map(move |number_entry2| {
                        symbols
                            .iter()
                            .filter_map(move |sym_entry| {
                                if sym_entry.is_gear_ratio(number_entry1, number_entry2) {
                                    let num1 = parse_from_lines(lines, number_entry1);
                                    let num2 = parse_from_lines(lines, number_entry2);
                                    Some(num1 * num2)
                                } else {
                                    None
                                }
                            })
                    })
            })
            .collect()
}