
fn main () {
    let input = include_str!("./part1.txt");
    let output = part1(input);
    println!("{output}")
}

fn part1(input: &str) -> String {
    let mut sum = 0;

    input
        .lines()
        .for_each(|line| sum += count_arrangements(line));

    sum.to_string()

}

fn count_arrangements(line: &str) -> usize {
    let splits: Vec<&str> = line.split(' ').collect();
    let string_sequence = splits[0];
    let corrupted_groups: Vec<usize> = splits[1]
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    make_arrangements(string_sequence, 0, &corrupted_groups).len()
}

fn is_valid(arrangement: &str, corrupted_groups: &[usize]) -> bool {
    let mut count = 0;
    let mut groups = Vec::new();
    
    for spring in arrangement.chars() {
        if spring == '#' {
            count += 1;
        } else if count > 0 {
            groups.push(count);
            count = 0;
        }
    }
    
    if count > 0 {
        groups.push(count);
    }
    
    groups == corrupted_groups
}

fn make_arrangements(
    sequence: &str,
    index: usize,
    corrupted_groups: &[usize],
    ) -> Vec<String> {
    
    if index >= sequence.len() {
        return if is_valid(sequence, corrupted_groups) {
            vec![sequence.to_string()]
        } else {
            vec![]
        };
    }

    let mut arrangements = Vec::new();
    let mut chars: Vec<char> = sequence.chars().collect();
    
    if chars[index] == '?' {
        chars[index] = '.';
        arrangements.append(&mut make_arrangements(
            &chars.iter().collect::<String>(),
            index + 1,
            corrupted_groups,
        ));
        chars[index] = '#';
        arrangements.append(&mut make_arrangements(
            &chars.iter().collect::<String>(),
            index + 1,
            corrupted_groups,
        ));
    } else {
        arrangements.append(&mut make_arrangements(sequence, index + 1, corrupted_groups));
    }
    arrangements
}