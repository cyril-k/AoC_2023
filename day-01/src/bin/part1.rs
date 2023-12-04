use regex::Regex;

fn main () {
    let input = include_str!("./part1.txt");
    let output = part1(input);
    println!("{output}")
}

fn part1(input: &str) -> String {
    let re = Regex::new(r"(\d{1})").unwrap();
    let mut numbers = vec![];
    for line in input.lines() {
        let matches: Vec<_> = re.find_iter(line).map(|m| m.as_str()).collect();
        let number = format!(
            "{}{}", 
            matches.first().unwrap(), 
            matches.last().unwrap()
        ).parse::<i64>().unwrap();
        numbers.push(number);
    }

    // println!("numbers: {:?}", numbers);
    numbers.iter().sum::<i64>().to_string()
}