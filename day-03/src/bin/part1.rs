use core::ops::Range;
 
fn main() {
    let input = include_str!("./part1.txt");
    let output = part1(input);
    println!("{output}")
}

fn part1(input: &str) -> String {
    let re_syms = regex::Regex::new(r"(\*|\/|\-|\+|\&|\=|\@|\$|\%|\#)").unwrap(); // */-+&=@$%#
    let re_nums = regex::Regex::new(r"(\d+)").unwrap();
    let mut prev_nums = vec![];
    let mut prev_syms = vec![];
    let mut prev_line: &str = "";
    let mut adjacent_nums: Vec<usize> = vec![];
    for (i, line) in input.lines().enumerate() {
        let syms: Vec<_> = re_syms.find_iter(line).map(|m| m.range()).collect();  
        let nums: Vec<_> = re_nums.find_iter(line).map(|m| m.range()).collect();  
        let mut prev_line_nums: Vec<usize> = if i > 0 {
            // first we check if nums from previous line are adjacent to syms on this line
            // ..35..633.
            // ......#...
            //       ^
            check_row(&syms, &prev_nums, prev_line.to_string())

        } else {vec![]};
        // next we will check if nums from this line are adjacent to syms on the previous line
        let mut prev_line_syms = check_row(&prev_syms, &nums, line.to_string());
        // finally we check the current line
        let mut this_line = check_row(&syms, &nums, line.to_string());

        adjacent_nums.append(&mut prev_line_nums);
        adjacent_nums.append(&mut prev_line_syms);
        adjacent_nums.append(&mut this_line);

        prev_nums = nums;
        prev_syms = syms;
        prev_line = line;
    }
    println!("adjacent nums: {:?}", adjacent_nums);
    // "output".to_string()
    adjacent_nums.iter().sum::<usize>().to_string()
}
 
fn do_intersect(a: &Range<usize>, b: &Range<usize>) -> bool {
    a.start <= b.end && a.end >= b.start
}

fn check_row(
        symbol_ranges: &Vec<Range<usize>>, 
        nums_ranges: &Vec<Range<usize>>, 
        line: String
    ) -> Vec<usize> {
    let mut parsed_nums: Vec<usize> = vec![];
    for sym in symbol_ranges.iter() {
        for num_val in nums_ranges.iter() {
            if do_intersect(sym, num_val) {
                let parsed_num = line[num_val.clone()]
                    .parse::<usize>()
                    .unwrap();
                parsed_nums.push(parsed_num);
            }
        };
    }
    parsed_nums
}