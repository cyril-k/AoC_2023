use core::cmp::Ordering;
use std::{cmp::{min, max}, collections::HashMap};
fn main () {
    let input = include_str!("./part1.txt");
    let output = part1(input);
    println!("{output}")
}

fn part1(input: &str) -> String {
    let mut blocks = Vec::new();

    for line in input.lines() {
        blocks.push(Block::new(line));
    }
    settle(&mut blocks);
    let unstable_blocks = check_stability(&mut blocks);
    (blocks.len() - unstable_blocks.len()).to_string()

    //"output".to_string()
}

fn settle(blocks: &mut Vec<Block>) -> HashMap<(usize, usize, usize), bool> {
    let mut grid: HashMap<(usize, usize, usize), bool> = HashMap::new();
    blocks.sort();

    for block in &mut *blocks {
        let (x1, y1, z1) = (block.vertex_a.0, block.vertex_a.1, block.vertex_a.2);
        let (x2, y2, z2) = (block.vertex_b.0, block.vertex_b.1, block.vertex_b.2);

        let (mut z1, mut z2) = (min(z1, z2), max(z1, z2));

        if x1 != x2 {
            let min_x = min(x1, x2);
            let max_x = max(x1, x2);
            while z1 > 0 && (min_x..=max_x).all(|x| !grid.contains_key(&(x, y1, z1 - 1))) {
                z1 -= 1;
                z2 -= 1;
            }
        } else if y1 != y2 {
            let min_y = min(y1, y2);
            let max_y = max(y1, y2);
            while z1 > 0 && (min_y..=max_y).all(|y| !grid.contains_key(&(x1, y, z1 - 1))) {
                z1 -= 1;
                z2 -= 1;
            }
        } else {
            while z1 > 0 && !grid.contains_key(&(x1, y1, z1 - 1)) {
                z1 -= 1;
                z2 -= 1;
            }
        }

        for x in min(x1, x2)..=max(x1, x2) {
            for y in min(y1, y2)..=max(y1, y2) {
                for z in z1..=z2 {
                    grid.insert((x, y, z), true);
                }
            }
        }

        // update original block
        block.vertex_a.2 = z1;
        block.vertex_b.2 = z2;
    }
    grid
}

fn check_stability(blocks: &mut Vec<Block>) -> Vec<usize> {
    let mut unstable_bricks = Vec::new();
    let original_grid = settle(&mut blocks.clone());

    for (i, block) in blocks.iter().enumerate() {
        let mut temp_blocks = blocks.clone();
        temp_blocks.remove(i); 
        let new_grid = settle(&mut temp_blocks);
        let mut original_grid_minus = original_grid.clone();
        for unit in block.to_units() {
            original_grid_minus.remove(&unit);
        }
        
        if new_grid != original_grid_minus {
            unstable_bricks.push(i);
        }
    }

    unstable_bricks
}


#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Vertex(usize, usize, usize);

impl Vertex {
    fn from_str(text: &str) -> Self {
        let coords = text.split(',').map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        Vertex(coords[0], coords[1], coords[2])
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Block {
    vertex_a: Vertex,
    vertex_b: Vertex,
}

impl Block {
    fn new(line: &str) -> Self {
        let vertices = line.split('~').collect::<Vec<&str>>();
        Block { 
            vertex_a: Vertex::from_str(vertices[0]), 
            vertex_b: Vertex::from_str(vertices[1]),
        }
    }

    fn to_units(&self) -> Vec<(usize, usize,usize)> {
        let (x1, y1, z1) = (self.vertex_a.0, self.vertex_a.1, self.vertex_a.2);
        let (x2, y2, z2) = (self.vertex_b.0, self.vertex_b.1, self.vertex_b.2);
        let (z1, z2) = (min(z1, z2), max(z1, z2));

        let mut units = Vec::new();

        for x in min(x1, x2)..=max(x1, x2) {
            for y in min(y1, y2)..=max(y1, y2) {
                for z in z1..=z2 {
                    units.push((x, y, z));
                }
            }
        }

        units
    }
}

impl Ord for Block {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_min_z = min(self.vertex_a.2, self.vertex_b.2);
        let other_min_z = min(other.vertex_a.2, other.vertex_b.2);
        self_min_z.cmp(&other_min_z)
    }
}

impl PartialOrd for Block {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}