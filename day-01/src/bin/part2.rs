use aho_corasick::AhoCorasick;

fn main () {
    let input = include_str!("./part1.txt");
    let output = part2(input);
    println!("{output}")
}

fn part2(input: &str) -> String {
    
    let mut digit_str  = (0..9)
        .map(|val| val.to_string())
        .collect::<Vec<String>>();
    let mut nums_str = ["one", "two", "three", "four", "five", "six", "seven", "ight", "nine"]
        .iter()
        .map(|&s| s.into())
        .collect::<Vec<String>>();
    nums_str.append(&mut digit_str);

    let ac = AhoCorasick::new(nums_str.iter()).unwrap();
    let mut numbers = vec![];
    for (i, line) in input.lines().enumerate() {
        let mut matches = vec![];
        for matched_pattern in ac.find_iter(line).map(|m| &nums_str[m.pattern().as_usize()]) {
            let number_str = match matched_pattern.as_str() {
                "zero" => Some("0"),
                "one" => Some("1"),
                "two" => Some("2"),
                "three" => Some("3"),
                "four" => Some("4"),
                "five" => Some("5"),
                "six" => Some("6"),
                "seven" => Some("7"),
                "ight" => Some("8"),
                "nine" => Some("9"),
                val => Some(val),
            };
            matches.push(number_str.unwrap());
        }

        println!("{}: {:?}", i, matches);
        let number = format!(
            "{}{}", 
            matches.first().unwrap(), 
            matches.last().unwrap()
        ).parse::<i64>().ok();
        numbers.push(number.unwrap());
    }

    println!("numbers: {:?}", numbers);
    numbers.iter().sum::<i64>().to_string()
    // "output".to_string()
}