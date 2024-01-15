use std::cmp::{min, max, Ordering};
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

fn main () {
    let input = include_str!("./part1.txt");
    let output = part1(input);
    println!("{output}")
}

fn part1(input: &str) -> String {
    let mut intersections = Vec::new();

    for line in input.lines() {
        let splits = line.split(" @ ").collect::<Vec<&str>>();
        let coords = splits[0].split(", ").map(|s| s.trim().parse().unwrap()).collect::<Vec<f64>>();
        let (x0, y0) = (coords[0], coords[1]);

        let deltas = splits[1].split(", ").map(|s| s.trim().parse().unwrap()).collect::<Vec<f64>>();
        let (dx, dy) = (deltas[0], deltas[1]);

        let args = to_linear((x0, dx, y0, dy));
        
        //let limits = (200000000000000.0, 400000000000000.0);
        let limits = (7.0, 27.0);
        
        println!("intersections: {:?}", line_rectangle_intersection(limits.0, limits.1, args.0, args.1, x0, y0));
        intersections.push(line_rectangle_intersection(limits.0, limits.1, args.0, args.1, x0, y0));
    }

    let mut counter = 0;

    for (i, int1) in intersections.iter().enumerate() {
        for int2 in intersections[i..].iter() {
            if they_intersect(int1, int2) {
                counter += 1;
            }
        }

    }

    // println!("{:?}", projectiles);
    //"output".to_string()
    counter.to_string()
}

fn they_intersect(
    p1: &Vec<(BadFloat, BadFloat)>, 
    p2: &Vec<(BadFloat, BadFloat)>, 
) -> bool {
    let (e1, x1) = (p1[0], p1[1]);
    let (e2, x2) = (p2[0], p2[1]);
    if e1.0 == x1.0 || e2.0 == x2.0 {
        return false
    }
    
    let horizontal_overlap = (min(e1.0, x1.0) < max(e2.0, x2.0)) &&
                             (min(e2.0, x2.0) < max(e1.0, x1.0));

    let vertical_overlap = (min(e1.1, x1.1) < max(e2.1, x2.1)) &&
                             (min(e2.1, x2.1) < max(e1.1, x1.1));

    horizontal_overlap && vertical_overlap
}


#[derive(Clone, Copy, Debug)]
struct BadFloat(f64);

impl PartialEq for BadFloat {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() < 1e-6
    }
}

impl Eq for BadFloat {}

impl Hash for BadFloat {
    fn hash<H: Hasher>(&self, state: &mut H) {
        ((self.0 * 1e6 as f64).round() as i64).hash(state);
    }
}

impl PartialOrd for BadFloat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.eq(other) {
            Some(Ordering::Equal)
        } else if self.0 < other.0 {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl Ord for BadFloat {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn to_linear(p: (f64, f64, f64, f64)) -> (f64, f64) {
    let k = p.3 / p.1;
    let b = p.2 - k * p.0;

    (k, b)
}

fn line_rectangle_intersection(
    min_lim: f64,
    max_lim: f64,
    k: f64,
    b: f64,
    x0: f64,
    y0: f64,
) -> Vec<(BadFloat, BadFloat)> {
    let mut intersections = Vec::new();

    let (x1, y1) = (min_lim, min_lim);
    let (x2, y2) = (max_lim, max_lim);

    let top_intersection_x = (y1 - b) / k;
    if top_intersection_x >= x1 && top_intersection_x <= x2 {
        intersections.push((BadFloat(top_intersection_x), BadFloat(y1)));
    }

    let bottom_intersection_x = (y2 - b) / k;
    if bottom_intersection_x >= x1 && bottom_intersection_x <= x2 {
        intersections.push((BadFloat(bottom_intersection_x), BadFloat(y2)));
    }

    let left_intersection_y = k * x1 + b;
    if left_intersection_y >= y1 && left_intersection_y <= y2 {
        intersections.push((BadFloat(x1), BadFloat(left_intersection_y)));
    }

    let right_intersection_y = k * x2 + b;
    if right_intersection_y >= y1 && right_intersection_y <= y2 {
        intersections.push((BadFloat(x2), BadFloat(right_intersection_y)));
    }

    intersections
}
