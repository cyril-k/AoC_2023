
fn main () {
    let input = include_str!("./part1.txt");
    let output = part1(input);
    println!("{output}")
}

fn part1(input: &str) -> String {
    let mut dirs = Vec::new();
    let mut lengths = Vec::new();

    for line in input.lines() {
        let (dir, length) = match line.split(' ').collect::<Vec<&str>>().as_slice() {
            [dir, length_s, ..] => (dir.chars().nth(0).unwrap(), length_s.parse::<isize>().unwrap()),
            _ => unreachable!(),
        };

        dirs.push(dir);
        lengths.push(length);
    };

    // let surface =  surface_a(&dirs, &lengths).to_string();

    let visualization = visualize_polygon(&dirs, &lengths);

    // format!("Surface Area: {}\nVisualization:\n{}", surface, visualization)
    println!("{visualization}");

    let filled_cells_count = visualization.chars().filter(|&c| c == '#').count();
    // "output".to_string()
    filled_cells_count.to_string()
}

struct Point(isize, isize);

fn surface_a(dirs: &Vec<char>, lengths: &Vec<isize>) -> isize {
    // let mut point = Point(0, 0);
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut surface: isize = 0;
    
    for (dir, length) in dirs.into_iter().zip(lengths) {
        // let new_point = map_point(&point, dir, length);
        // surface += new_point.0 * point.1 - point.0 * new_point.1;
        // point = new_point;
        // let length = length +1;

        
        if *dir == 'L' {
            x -= length;
            surface -= length * (y);
        } else if *dir == 'R' {
            x += length;
            surface += length * (y);
        } else if *dir == 'U' {
            y -= length;
        } else if *dir == 'D' {
            y += length;
        }
        
        println!("x: {}", x);
        println!("y: {}", y);
        println!("surface:{}", surface);
    }

    surface.abs()
}


fn visualize_polygon(dirs: &Vec<char>, lengths: &Vec<isize>) -> String {
    let (mut x, mut y) = (0, 0);
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;

    for (dir, length) in dirs.iter().zip(lengths.iter()) {
        match dir {
            'L' => x -= length,
            'R' => x += length,
            'U' => y += length,
            'D' => y -= length,
            _ => unreachable!(),
        }
        min_x = min_x.min(x);
        max_x = max_x.max(x);
        min_y = min_y.min(y);
        max_y = max_y.max(y);
    }

    let width = (max_x - min_x + 1) as usize;
    let height = (max_y - min_y + 1) as usize;
    let mut grid = vec![vec![0; width]; height];

    x = 0;
    y = 0;

    for (dir, length) in dirs.iter().zip(lengths.iter()) {
        let (dx, dy) = match dir {
            'L' => (-1, 0),
            'R' => (1, 0),
            'U' => (0, 1),
            'D' => (0, -1),
            _ => unreachable!(),
        };
        for _ in 0..*length {
            x += dx;
            y += dy;
            grid[(y - min_y) as usize][(x - min_x) as usize] = 1;
        }
    }

    let start_x = 100; // dodgy
    let start_y = 100;
    fill_polygon(&mut grid, start_x, start_y, width - 1, height - 1);

    let mut grid = grid.iter()
        .map(|row| row.iter().map(|&cell| if cell == 1 { '#' } else { '.' }).collect::<String>())
        .collect::<Vec<String>>();
        
    grid.reverse();

    grid.join("\n")
}


fn fill_polygon(grid: &mut Vec<Vec<i32>>, x: usize, y: usize, max_x: usize, max_y: usize) {
    if x > max_x || y > max_y || grid[y][x] != 0 {
        return;
    }

    grid[y][x] = 1;

    fill_polygon(grid, x + 1, y, max_x, max_y);
    fill_polygon(grid, x, y + 1, max_x, max_y);
    fill_polygon(grid, x.wrapping_sub(1), y, max_x, max_y);
    fill_polygon(grid, x, y.wrapping_sub(1), max_x, max_y);
}


// fn map_point(p: &Point, dir: char, length: isize) -> Point {
//     match dir {
//         'U' => Point(p.0, p.1 + length),
//         'D' => Point(p.0, p.1 - length),
//         'L' => Point(p.0 - length, p.1),
//         'R' => Point(p.0 + length, p.1),
//         _ => unreachable!(),
//     }
// }