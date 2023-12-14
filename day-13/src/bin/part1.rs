use ndarray::{self, Array2, s};

fn main () {
    let input = include_str!("./part1.txt");
    let output = part1(input);
    println!("{output}")
}

fn part1(input: &str) -> String {
    let mut results = Vec::new();
    for text in input.split("\r\n\r\n") {
        let lines: Vec<&str> = text.lines().collect();
        let rows = lines.len();
        let cols = lines[0].chars().count();

        let _array = lines
            .iter()
            .flat_map(|line| {
                line.chars().map(move |c| {
                    new_pixel(c)
                })
            })
            .collect::<Vec<usize>>();
        let array = Array2::from_shape_vec((rows, cols), _array)
        .unwrap();

        results.push(find_reflection(array));
    }

    // "output".to_string()
    results.iter().sum::<usize>().to_string()
}

fn new_pixel(c: char) -> usize {
    match c {
        '#' => 1,
        _ => 0,
    }
}

fn find_reflection(array: Array2<usize>) -> usize {
    let (_, n_cols) = array.dim();
    let mut widths = Vec::new();

    for l in 0..n_cols {
        let mut r = n_cols-1;
        while l < r {
            //if n elements is not even, decrease r
            if (r - l + 1) % 2 != 0 {
                r -= 1;
            }
            let middle = ((r-l) / 2) + l;
            let left_half = array.slice(s![.., l..=middle]);
            let right_half = array.slice(s![.., middle+1..=r]);
            if compare_arrays(left_half.to_owned(), right_half.to_owned()) 
                && (l==0 || r == n_cols-1) {
                widths.push(middle+1);

            }
            r -= 1;
        }
    }

    let max = match widths.into_iter().max() {
            Some(num) => num,
            None => find_reflection(array.t().to_owned())*100,
        };
    
    max
}

fn compare_arrays(a1:Array2<usize>, mut a2:Array2<usize>) -> bool {
    // flip a2 across columns
    let n_cols = a2.ncols();
    for mut row in a2.axis_iter_mut(ndarray::Axis(0)) {
        for i in 0..(n_cols / 2) {
            row.swap(i, n_cols - 1 - i);
        }
    }
    a1 == a2

}

