use std::cmp::{min, max};
fn main () {
    let input = include_str!("./part1.txt");
    let output = part1(input);
    println!("{output}")
}

fn part1(input: &str) -> String {

    for line in input.lines() {
        
    }

    "output".to_string()
}

struct Vertex(usize, usize, usize);

impl Vertex {
    fn from_str(text: &str) -> Self {
        let coords = text.split(',').map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        Vertex(coords[0], coords[1], coords[2])
    }
}

struct Block {
    vertex_a: Vertex,
    vertex_b: Vertex,
}

fn range_overlap(a1: usize, a2: usize, b1: usize, b2: usize) -> bool {
    max(min(a1, a2), min(b1, b2)) <= min(max(a1, a2), max(b1, b2))
}

impl Block {
    fn new(line: &str) -> Self {
        let vertices = line.split('~').collect::<Vec<&str>>();
        Block { 
            vertex_a: Vertex::from_str(vertices[0]), 
            vertex_b: Vertex::from_str(vertices[1]),
        }
    }


    fn is_horizontal(&self) -> bool {
        self.vertex_a.1 == self.vertex_b.1
    }

    fn is_vertical(&self) -> bool {
        self.vertex_a.0 == self.vertex_b.0
    }

    fn cross_segment(&self, other: &Block) -> Option<(usize, usize)> {
        if self.is_horizontal() {
            let candidate_insc = (other.vertex_a.0, self.vertex_a.1);
            if range_overlap(self.vertex_a.0, self.vertex_b.0, other.vertex_a.0, other.vertex_b.0) &&
               range_overlap(other.vertex_a.1, other.vertex_b.1, self.vertex_a.1, self.vertex_b.1) {
                return Some(candidate_insc);
            }
        } else {
            let candidate_insc = (self.vertex_a.0, other.vertex_a.1);
            if range_overlap(self.vertex_a.1, self.vertex_b.1, other.vertex_a.1, other.vertex_b.1) &&
               range_overlap(other.vertex_a.0, other.vertex_b.0, self.vertex_a.0, self.vertex_b.0) {
                return Some(candidate_insc);
            }
        }
        None
    }

    fn check_collinear(&self, other: &Block) -> Option<(usize, usize)> {
        if self.is_horizontal() && other.is_horizontal() {
            if range_overlap(self.vertex_a.0, self.vertex_b.0, other.vertex_a.0, other.vertex_b.0) {
                return Some((0, self.vertex_a.1)); 
            }
        } else if self.is_vertical() && other.is_vertical() {
            if range_overlap(self.vertex_a.1, self.vertex_b.1, other.vertex_a.1, other.vertex_b.1) {
                return Some((self.vertex_a.0, 0)); 
            }
        }
        None
    }

    fn intersects_with(&self, other: &Block) -> Option<(usize, usize)> {
        if self.is_horizontal() != other.is_horizontal() {
            self.cross_segment(other)
        } else {
            self.check_collinear(other)
        }
    }
}

