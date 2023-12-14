use std::collections::HashSet;
use ndarray::{self, Array2};


fn main () {
    let input = include_str!("./part1.txt");
    let output = part1(input);
    println!("{output}")
}

fn part1(input: &str) -> String {
    // for line in input.lines() {
    let lines: Vec<&str> = input.lines().collect();
    let rows = lines.len();
    let cols = lines[0].chars().count();

    let maze_array = lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                let pointer = Pointer { x, y };
                Tile::new(c, pointer)
            })
        })
        .collect::<Vec<Tile>>();

    let maze_array = Array2::from_shape_vec((rows, cols), maze_array)
        .expect("Failed to reshape into Array2");

    let maze = Maze::new(maze_array);
    let count_steps = maze.traverse_maze();


    ((count_steps+1)/2).to_string()
    // "output".to_string()
}

#[derive(Eq, PartialEq, Hash, Debug)]
enum Directions {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Tile {
    layout: HashSet<Directions>,
    start: bool,
    pointer: Pointer,
}

impl Tile {
    fn new(c: char, pointer: Pointer) -> Self {
        let (layout, start) = match c {
            '|' => (HashSet::from([Directions::Up, Directions::Down]), false),
            '-' => (HashSet::from([Directions::Right, Directions::Left]), false),
            'J' => (HashSet::from([Directions::Up, Directions::Left]), false),
            'L' => (HashSet::from([Directions::Up, Directions::Right]), false),
            '7' => (HashSet::from([Directions::Down, Directions::Left]), false),
            'F' => (HashSet::from([Directions::Down, Directions::Right]), false),
            'S' => (HashSet::from([
                Directions::Up, 
                Directions::Down, 
                Directions::Left, 
                Directions::Right
                ]), true),
            _ => (HashSet::new(), false),
        };

        Self { 
            layout, 
            start, 
            pointer
         }
    }


    fn is_connected_to(&self, other: &Tile) -> bool {
        // Calculate the difference in coordinates between two tiles
        let (self_pos, other_pos) = (&self.pointer, &other.pointer);
        let (dx, dy) = ((other_pos.x as isize) - (self_pos.x as isize), (other_pos.y as isize) - (self_pos.y as isize));
        match dx {
            -1 => {
                // self to the right of other
                self.layout.contains(&Directions::Left) && other.layout.contains(&Directions::Right)
            },
            1 => {
                // self to the left of other
                self.layout.contains(&Directions::Right) && other.layout.contains(&Directions::Left)
            },
            0 => match dy {
                -1 => {
                    // self below other
                    self.layout.contains(&Directions::Up) && other.layout.contains(&Directions::Down)
                },
                1 => {
                    // self above other
                    self.layout.contains(&Directions::Down) && other.layout.contains(&Directions::Up)
                },
                _ => false,
            },
            _ => false,
        }
       
    }
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct Pointer {
    x: usize, // does it actually use less space on the stack than plain value?
    y: usize,
}

impl Pointer {
    fn from_index(index: (usize, usize)) -> Self {
        Self { x: index.1, y: index.0 }
    }

    fn to_index(&self) -> (usize, usize) {
        (self.y, self.x)
    }

    fn advance(&self) -> Vec<Pointer> {
        let index = self.to_index();
        let up = Pointer::from_index((index.0, index.1+1));
        let down = Pointer::from_index((index.0, index.1.checked_sub(1).unwrap_or(0)));
        let left = Pointer::from_index((index.0.checked_sub(1).unwrap_or(0), index.1));
        let right = Pointer::from_index((index.0+1, index.1));
        vec![up, down, left, right]
    }

}
struct Maze {
    body: Box<Array2<Tile>>,
    start: Pointer,
}

impl Maze {
    fn new(array_2d: Array2<Tile>) -> Self {

        let start_index = array_2d
            .indexed_iter()
            .find_map(|(index, tile)| {
                if tile.start {
                    Some(index)
                } else {
                    None
                }
            });

        let start = Pointer::from_index(start_index.unwrap());

        Self { 
            body: Box::new(array_2d), 
            start, 
        }

    }

    fn get_by_index(&self, index: (usize, usize)) -> Option<&Tile> {
        self.body.get(index)
    } 

    fn traverse_maze(&self) -> usize {
        // consider surrounding for the starting point
        self.start
            .advance()
            .into_iter()
            .map(|p| {
                // follow the entrypoint 
                // println!("looking at option {:?}", p.to_index());
                self._traverse_option(p).len()
            })
            .max()
            .unwrap()

    }
        
    fn _traverse_option(&self, p: Pointer) -> Vec<Pointer> {
        let memory: Box<HashSet<Pointer>> = Box::new(HashSet::new());
        self._traverse(p, memory).unwrap()
    }
        
    // Recursion overflows the stack
    
    // fn _traverse_r(
    //     &self,
    //     p: Pointer,
    //     mut memory: Box<HashSet<Pointer>>,
    // ) -> Option<Vec<Pointer>> {
    //     println!("values in memory {:?}", memory.len());
    //     let this_tile = self.get_by_index(p.to_index())?;
    //     let candidates = p.advance();
    //     memory.insert(p);
    //     for candidate in candidates.into_iter().filter(|p| !memory.contains(p) && p != &self.start) {
    //         let candidate_tile = self.get_by_index(candidate.to_index());
    //         match candidate_tile {
    //             Some(other_tile) => {
    //                 if this_tile.is_connected_to(other_tile) { 
    //                     if candidate != self.start { return self._traverse(candidate, memory); }
                       
    //                 }},
    //                     None => (), // reached dead end and touched the end of the map
    //         }
    //     }
    //     Some(memory.into_iter().collect::<Vec<Pointer>>())
    // }
                                    
    fn _traverse(
        &self,
        p: Pointer,
        mut memory: Box<HashSet<Pointer>>,
    ) -> Option<Vec<Pointer>> {
        let mut to_explore = vec![p];
        
        while !to_explore.is_empty() {
            let current = to_explore.pop()?;
            let this_tile = self.get_by_index(current.to_index())?;
            let candidates = current.advance();
            
            memory.insert(current);
            
            // if at some point we come bact to the start, here we go
            for candidate in candidates.into_iter().filter(|c| !memory.contains(c) && c != &self.start) {
                let candidate_tile = self.get_by_index(candidate.to_index());
                match candidate_tile {
                    Some(other_tile) => {
                        if this_tile.is_connected_to(other_tile) {
                            if candidate != self.start {
                                to_explore.push(candidate);
                                break;
                            }
                        }
                    },
                    None => (), // reached dead end and touched the end of the map
                }
            }
        }
    
        Some(memory.into_iter().collect::<Vec<Pointer>>())
    }



}