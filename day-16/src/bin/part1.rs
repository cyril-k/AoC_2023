use std::collections::HashSet;
use ndarray::{self, Array2};

use csv::Writer;
use std::error::Error;
use std::fs::File;


fn main () {
    let input = include_str!("./part1.txt");
    let output = part1(input);
    println!("{output}")
}

fn part1(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let rows = lines.len();
    let cols = lines[0].chars().count();

    let array = Array2::from_shape_vec(
        (rows, cols), 
        lines
            .iter()
            .flat_map(|line| line.chars().map(Tile::new))
            .collect())
        .expect("Failed to reshape into 2D array");

    let count = trace_beam(array);

    count.to_string()
    // "output".to_string()
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
enum Tile {
    SplitterH(bool), // '-'
    SplitterV(bool), // '|'
    MirrorSlash(bool, bool, bool, bool), // '/'
    MirrorAntiSlash(bool, bool, bool, bool), // '\'
    Nil // '.'
}

impl Tile {
    fn new(c: char) -> Self {
        match c {
            '-' => Tile::SplitterH(false),
            '|' => Tile::SplitterV(false),
            '/' => Tile::MirrorSlash(false, false, false, false), 
            '\\' => Tile::MirrorAntiSlash(false, false, false, false), 
            _ => Tile::Nil,
        }
    }

    fn to_char(&self) -> char {
        match self {
            Tile::SplitterH(_) => '-',
            Tile::SplitterV(_) => '|',
            Tile::MirrorSlash(..) => '/',
            Tile::MirrorAntiSlash(..) => '\\',
            Tile::Nil => '.',
        }
    }

    fn transfer(&mut self, source_dir: Direction) -> Vec<Direction> {
        match self {
            Tile::SplitterH(activated) => match source_dir {
                    Direction::Up | Direction::Down => if !*activated {
                        *activated = true;
                        vec![Direction::Left, Direction::Right]
                    } else { vec![] },
                    _ => vec![source_dir],
                },
            Tile::SplitterV(activated) => match source_dir {
                    Direction::Left | Direction::Right => if !*activated {
                        *activated = true;
                        vec![Direction::Up, Direction::Down] 
                    } else { vec![] },
                    _ => vec![source_dir],
                },
            Tile::MirrorSlash(
                side_up, 
                side_down, 
                side_left, 
                side_right
            ) => match source_dir {
                Direction::Up if !*side_up => {
                    *side_up = true;
                    vec![Direction::Right]
                },
                Direction::Down if !*side_down => {
                    *side_down = true;
                    vec![Direction::Left]
                },
                Direction::Left if !*side_left => {
                    *side_left = true;
                    vec![Direction::Down]
                },
                Direction::Right if !*side_right => {
                    *side_right = true;
                    vec![Direction::Up]
                },
                _ => vec![],
            },
            Tile::MirrorAntiSlash(
                side_up, 
                side_down, 
                side_left, 
                side_right
            ) => match source_dir {
                Direction::Up if !*side_up => {
                    *side_up = true;
                    vec![Direction::Left]
                },
                Direction::Down if !*side_down => {
                    *side_down = true;
                    vec![Direction::Right]
                },
                Direction::Left if !*side_left => {
                    *side_left = true;
                    vec![Direction::Up]
                },
                Direction::Right if !*side_right => {
                    *side_right = true;
                    vec![Direction::Down]
                },
                _ => vec![],
            },
            Tile::Nil => vec![source_dir],
        }
    }
}


#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl std::ops::Not for Direction {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

fn trace_beam(mut array:Array2<Tile>) -> usize {
    let mut to_explore: Vec<Beam> = Vec::new();
    let start = Beam::from_index((0, 0), Direction::Right);
    update_beams(&mut array, start, &mut to_explore);
    
    let mut explored: HashSet<(usize, usize)> = HashSet::new();

    while let Some(mut beam) = to_explore.pop() {
        explored.insert(beam.to_index());
        if beam.advance(array.dim()).is_some() {    
            update_beams(&mut array, beam, &mut to_explore);
        }
    }
    explored.len()
}

fn update_beams(array: &mut Array2<Tile>, mut beam: Beam, to_explore: &mut Vec<Beam>) {
    let tile = array.get_mut(beam.to_index()).unwrap();
    for new_dir in tile.transfer(beam.direction) {
        beam.deflect(new_dir);
        to_explore.push(beam.clone());
    }
}

fn viz(array: &Array2<Tile>, explored: &HashSet<(usize, usize)>) {
    let mut n_array: Array2<char> = array.mapv((|t| t.to_char()));
    for index in explored {
        *n_array.get_mut(*index).unwrap() = '#';
    }
    println!("{:?}", n_array);
    write_array2_to_csv(&n_array, "output.csv");
}

fn write_array2_to_csv(array: &Array2<char>, file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_writer(File::create(file_path)?);

    for row in array.genrows().into_iter() {
        wtr.serialize(row.to_vec())?;
    }

    wtr.flush()?;
    Ok(())
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
struct Beam {
    x: usize, 
    y: usize,
    direction: Direction,
}

impl Beam {
    fn from_index(index: (usize, usize), dir: Direction) -> Self {
        Self { x: index.1, y: index.0, direction: dir }
    }

    fn to_index(&self) -> (usize, usize) {
        (self.y, self.x)
    }

    fn advance(&mut self, limits: (usize, usize)) -> Option<Self> {
        match self.direction {
            Direction::Up => self.y = self.y.checked_sub(1)?,
            Direction::Down if self.y + 1 != limits.0 => self.y += 1,
            Direction::Left => self.x = self.x.checked_sub(1)?,
            Direction::Right if self.x + 1 != limits.1 => self.x += 1,
            _ => return None,
        };
        Some(*self)
    }

    fn deflect(&mut self, dir: Direction) {
        self.direction = dir;
    }

}
