use std::cmp::{min, max};
use std::collections::HashSet;

fn main () {
    let input = include_str!("./part1.txt");
    let output = part1(input);
    println!("{output}")
}

fn part1(input: &str) -> String {
    // let mut projectiles = Vec::new();

    for line in input.lines() {
        let splits = line.split(" @ ").collect::<Vec<&str>>();
        let coords = splits[0].split(", ").map(|s| s.trim().parse().unwrap()).collect::<Vec<f64>>();
        let (x0, y0) = (coords[0], coords[1]);

        let deltas = splits[1].split(", ").map(|s| s.trim().parse().unwrap()).collect::<Vec<f64>>();
        let (dx, dy) = (deltas[0], deltas[1]);

        // projectiles.push((x0, dx, y0, dy));

        let args = to_linear((x0, dx, y0, dy));

        println!("args: {:?}", args);
    }

    // println!("{:?}", projectiles);
    "output".to_string()
}

struct Projectile {
    x0: f64,
    y0: f64,
    k: f64,
    b: f64,
}

impl Projectile {
    fn new(x0: f64, dx: f64, y0: f64, dy: f64) -> Self {
        Self { 
            x0, 
            y0, 
            k: dy/dx, 
            b: y0 - (dy/dx) * x0 
        }
    }
}


fn they_intersect(
    p1: (f64, f64, f64, f64), 
    p2: (f64, f64, f64, f64), 
    limits: (f64, f64)
) -> bool {
    
    // find if the origin is less, inside, or greater than limit
    
    
    false
}


fn to_linear(p: (f64, f64, f64, f64)) -> (f64, f64) {
    let k = p.3 / p.1;
    let b = p.2 - k * p.0;

    (k, b)
}

fn line_rectangle_intersection(
    min_lim: (f64, f64),
    max_lim: (f64, f64),
    k: f64,
    b: f64,
    x0: f64,
    y0: f64,
) -> HashSet<(f64, f64)> {
    let mut intersections = HashSet::new();

    let (x1, y1) = min_lim;
    let (x2, y2) = max_lim;

    // Check intersection with the top side of the rectangle
    let top_intersection_x = (y1 - b) / k;
    if top_intersection_x >= x1 && top_intersection_x <= x2 {
        intersections.insert((top_intersection_x, y1));
    }

    // Check intersection with the bottom side of the rectangle
    let bottom_intersection_x = (y2 - b) / k;
    if bottom_intersection_x >= x1 && bottom_intersection_x <= x2 {
        intersections.insert((bottom_intersection_x, y2));
    }

    // Check intersection with the left side of the rectangle
    let left_intersection_y = k * x1 + b;
    if left_intersection_y >= y1 && left_intersection_y <= y2 {
        intersections.insert((x1, left_intersection_y));
    }

    // Check intersection with the right side of the rectangle
    let right_intersection_y = k * x2 + b;
    if right_intersection_y >= y1 && right_intersection_y <= y2 {
        intersections.insert((x2, right_intersection_y));
    }

    intersections
}
