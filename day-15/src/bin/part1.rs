fn main () {
    let input = include_str!("./part1.txt");
    let output = part1(input);
    println!("{output}")
}

fn part1(input: &str) -> String {
    let mut sum:usize = 0;
    for step in input.split(',') { 
        let mut val: usize = 0;
        for c in step.chars().filter(|c| *c != '\n') {
            val = compute_hash(c, val);
        }
        sum += val;
    }
    // "output".to_string()
    sum.to_string()

}

fn compute_hash(c:char, mut val: usize) -> usize {
    val += c as usize;
    val *= 17;
    val %= 256;
    val
}