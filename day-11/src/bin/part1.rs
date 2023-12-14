use ndarray::{self, Array1, Array2, Axis};


fn main () {
    let input = include_str!("./part1.txt");
    let output = part1(input);
    println!("{output}")
}

fn part1(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let rows = lines.len();
    let cols = lines[0].chars().count();

    let sky_array = lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                new_pixel(c)
            })
        })
        .collect::<Vec<usize>>();

    let sky_array = Array2::from_shape_vec((rows, cols), sky_array)
        .expect("Failed to reshape into Array2");

    let sky = Sky::new(sky_array);

    // "output".to_string()
    sky.find_paths().to_string()

}



fn new_pixel(c: char) -> usize {
    match c {
        '#' => 1,
        _ => 0,
    }
}


#[derive(Eq, PartialEq, Hash, Debug)]
struct Pointer {
    x: usize, 
    y: usize,
}

impl Pointer {
    fn from_index(index: (usize, usize)) -> Self {
        Self { x: index.1, y: index.0 }
    }

    fn manhattan_d(&self, other: &Pointer) -> usize {
        absolute_difference(self.x, other.x) + absolute_difference(self.y, other.y)
    }

}

struct Sky {
    array: Array2<usize>,
    elements: Vec<Pointer>,
}

impl Sky {
    fn new(sky_array: Array2<usize>) -> Sky {
        let mut elements: Vec<Pointer> = Vec::new();
        let (n_rows, n_cols) = sky_array.dim();
        // expand empty columns
        let mut empty_cols_i: Vec<usize>= vec![]; 
        let mut empty_rows_i: Vec<usize>= vec![]; 
        for (i, col) in sky_array.axis_iter(Axis(1)).enumerate() {
            if col.iter().all(|p| *p==0) {
                empty_cols_i.push(i)
            }
        }
        for (i, row) in sky_array.axis_iter(Axis(0)).enumerate() {
            if row.iter().all(|p| *p==0) {
                empty_rows_i.push(i)
            }
        }

        let empty_cols_i = convert_indices(empty_cols_i);
        let empty_rows_i = convert_indices(empty_rows_i);
        let mut with_cols = Array2::from_elem((n_rows, n_cols + empty_cols_i.len()), new_pixel('.'));
        let col_to_insert = Array1::from_elem(n_rows, new_pixel('.'));
        
        let mut orig_col_idx = 0;
        for i in 0..(n_cols + empty_cols_i.len()) {
           
            if empty_cols_i.contains(&i) {
                with_cols.column_mut(i).assign(&col_to_insert);
            } else {
                with_cols.column_mut(i).assign(&sky_array.column(orig_col_idx));
                orig_col_idx += 1;

            }
        }

        // expand empty rows
        let mut with_rows = Array2::from_elem((n_rows + empty_rows_i.len(), n_cols + empty_cols_i.len()), new_pixel('.'));
        let row_to_insert = Array1::from_elem(n_cols + empty_cols_i.len(), new_pixel('.'));
        
        let mut orig_row_idx = 0;
        for i in 0..(n_rows + empty_rows_i.len()) {

            if empty_rows_i.contains(&i) {
                with_rows.row_mut(i).assign(&row_to_insert);
            } else {
                with_rows.row_mut(i).assign(&with_cols.row(orig_row_idx));
                orig_row_idx += 1;

            }
        }

        with_rows
            .indexed_iter()
            .for_each(|(index, pixel)| {
                if *pixel == 1 {
                    elements.push(Pointer::from_index(index));
                } 
            });


        Self { array: with_cols, elements }
    }

    fn find_paths(&self) -> usize {
        let mut pairs: Vec<(&Pointer, &Pointer)> = Vec::new();
        for i in 0..self.elements.len() {
            for j in i+1..self.elements.len() {
                pairs.push((&self.elements[i], &self.elements[j]))
            }
        }

        pairs
            .iter()
            .map(|(p1, p2)| {
                p1.manhattan_d(p2)
            })
            .sum()
    }

}

fn convert_indices(input: Vec<usize>) -> Vec<usize> {
    input
        .iter()
        .enumerate()
        .map(|(i, elem)| {
            let prev = if i > 0 { i+1 } else { 0 };
            prev + elem
        })
        .collect::<Vec<usize>>()
}

fn absolute_difference(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}
