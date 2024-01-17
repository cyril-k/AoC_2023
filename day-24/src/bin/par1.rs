
fn main () {
    let input = include_str!("./part1.txt");
    let output = part1(input);
    println!("{output}")
}

fn part1(input: &str) -> String {
    let mut projectiles = Vec::new();

    for line in input.lines() {
        let splits = line.split(" @ ").collect::<Vec<&str>>();
        let coords = splits[0].split(", ").map(|s| s.trim().parse().unwrap()).collect::<Vec<f64>>();
        let (x0, y0) = (coords[0], coords[1]);

        let deltas = splits[1].split(", ").map(|s| s.trim().parse().unwrap()).collect::<Vec<f64>>();
        let (dx, dy) = (deltas[0], deltas[1]);

        projectiles.push(Projectile::new(x0, dx, y0, dy));
    }

    let limits = (200000000000000.0, 400000000000000.0);
    // let limits = (7.0, 27.0);

    let mut counter = 0;

    for (i, p1) in projectiles.iter().enumerate() {
        for p2 in projectiles[i..].iter() {
            if let Some(coordinate) = p1.intersects_with(p2) {
                if coordinate.0 >= limits.0
                    && coordinate.0 <= limits.1
                    && coordinate.1 >= limits.0
                    && coordinate.1 <= limits.1
                {
                    counter += 1;
                }
            }
        }

    }

    //"output".to_string()
    counter.to_string()
}

struct Projectile {
    position: (f64, f64),
    velocity: (f64, f64),
    a: f64, // ax + by = c
    b: f64,
    c: f64,
}

impl Projectile {
    fn new(
        x: f64,
        dx: f64,
        y: f64,
        dy: f64,
    ) -> Self {
        Self { 
            position: (x, y), 
            velocity:(dx, dy), 
            a: dy, 
            b: -dx, 
            c: x * dy - y * dx 
        }
    }

    fn intersects_with(&self, other: &Projectile) -> Option<(f64, f64)> {
        let x = (other.b * self.c - self.b * other.c) / (self.a * other.b - self.b * other.a);
        let y = (self.a * other.c - other.a * self.c) / (self.a * other.b - self.b * other.a);

        let future = (x - self.position.0).signum() == self.velocity.0.signum()
            && (y - self.position.1).signum() == self.velocity.1.signum()
            && (x - other.position.0).signum() == other.velocity.0.signum()
            && (y - other.position.1).signum() == other.velocity.1.signum();

        if future {
            Some((x, y))
        } else {
            None
        }
    }
}
