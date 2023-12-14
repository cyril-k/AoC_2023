
fn main () {
    let input = include_str!("./part1.txt");
    let output = part1(input);
    println!("{output}")
}

fn part1(input: &str) -> String {
    for line in input.lines() {
        let splits = line.split(' ').collect::<Vec<&str>>();

        let (corrupted_s, groups) = match splits.as_slice() {
            &[first, second, ..] => (first, second),
            _ => unreachable!(),
        };

        let groups = groups
            .split(',')
            .filter_map(|c| c.parse::<usize>().ok())
            .collect::<Vec<usize>>();

        println!("corrupted_string {}", corrupted_s);
        println!("groups {:?}", groups);

        // for (i, group) in groups 

    }
    // let lines: Vec<&str> = input.lines().collect();
    // let rows = lines.len();
    // let cols = lines[0].chars().count();

    // let sky_array = lines
    //     .iter()
    //     .enumerate()
    //     .flat_map(|(y, line)| {
    //         line.chars().enumerate().map(move |(x, c)| {
    //             new_pixel(c)
    //         })
    //     })
    //     .collect::<Vec<usize>>();

    // let sky_array = Array2::from_shape_vec((rows, cols), sky_array)
    //     .expect("Failed to reshape into Array2");

    // let sky = Sky::new(sky_array);

    "output".to_string()

}

// struct BrokenSeq {
//     corrupted: String,
//     groups: Vec<usize>,
// }

// impl BrokenSeq {
//     fn find_combinations(&self) {
//         let mut something: Vec<usize> = vec![];
//         for (i, group) in self.groups {
//             // groups = [3,1,1]
//             0..x + x..x+3 + x+3+y..x+3+y+1
//         }
//     }
// }
