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
        let (syms, nums) = (
            re_syms.find_iter(line).map(|m| m.range()).collect::<Vec<_>>(),
            re_nums.find_iter(line).map(|m| m.range()).collect::<Vec<_>>(),
        );  
        
        let mut nums_on_prev: Vec<usize> = if i > 0 {
            // first we check if nums from previous line are adjacent to syms on this line:
            //       vvv
            // ..35..633.
            // ......#...

            check_row(&syms, &prev_nums, prev_line)
        } else { vec![] };
        // next we will check if nums from this line are adjacent to syms on the previous line:
        // ...*......
        // ..35..633.
        //   ^^

        let mut syms_on_prev = check_row(&prev_syms, &nums, line);
        // finally we check the current line:
        // 617*......
        // ^^^
        let mut this_line = check_row(&syms, &nums, line);

        adjacent_nums.append(&mut nums_on_prev);
        adjacent_nums.append(&mut syms_on_prev);
        adjacent_nums.append(&mut this_line);

        prev_nums = nums;
        prev_syms = syms;
        prev_line = line;
    }
    println!("adjacent nums: {:?}", adjacent_nums);
    // "output".to_string()
    adjacent_nums.iter().sum::<usize>().to_string()
}
 
fn are_adjacent(a: &Range<usize>, b: &Range<usize>) -> bool {
    a.start <= b.end && a.end >= b.start
}

fn check_row(
    symbol_ranges: &Vec<Range<usize>>,
    nums_ranges: &Vec<Range<usize>>,
    line: &str,
) -> Vec<usize> {
    symbol_ranges
        .iter()
        .flat_map(|sym| {
            nums_ranges.iter()
                .filter_map(|num_val| {
                    if are_adjacent(sym, num_val) {
                        line[num_val.clone()].parse::<usize>().ok()
                    } else {
                        None
                    }
                })
        })
        .collect()
}