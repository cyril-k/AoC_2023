use std::collections::HashMap;



fn main () {
    let input = include_str!("./part1.txt");
    let output = part1(input);
    println!("{output}")
}

fn part1(input: &str) -> String {

    for line in input.lines() {
        let splits: Vec<_> = line.split(" -> ").collect();

        if splits[0].starts_with('%') {
            let output_mod = Box::new(FlipFlop::new());
        } else if splits[0].starts_with('&') {
            let output_mod = Box::new(Conjuction::new());
        } else {
            let output_mod = Box::new(Broadcaster::new());
        }

        // match output_mod {
            
        // }
        
    }
    "output".to_string()
    }

trait Module {
    fn transfer(&mut self, input: Vec<bool>) -> Vec<bool>;
}
struct FlipFlop {
    state: bool
}

impl FlipFlop {
    fn new() -> Self {
        FlipFlop { state: false }
    }
}

impl Module for FlipFlop {
    fn transfer(&mut self, input: Vec<bool>) -> Vec<bool> {
        let mut output = Vec::new();
        for input_val in input {
            if !input_val {
                if !self.state {
                    self.state = true;
                    output.push(true)
                } else {
                    self.state = false;
                    output.push(false)
                }
            }
            output.push(true)
        }
        output
    }
}

struct ConjuctionUnit {
    unit_state: bool
}

impl ConjuctionUnit {
    fn new() -> Self {
        ConjuctionUnit { unit_state: false }
    }

    fn update(&mut self, input: bool) {
        self.unit_state = input;
    }    
}

struct Conjuction {
    units: Vec<ConjuctionUnit>
}

impl Conjuction {
    fn new() -> Self {
        Conjuction { units: Vec::new() }
    }
}

impl Module for Conjuction {
    fn transfer(&mut self, input: Vec<bool>) -> Vec<bool> {
        for (unit, signal) in self.units.iter_mut().zip(input) {
            unit.update(signal);
        }
        vec![!self.units.iter().all(|u| u.unit_state )]
    }
}

struct Broadcaster {
    outputs: Vec<String>
}

impl Broadcaster {
    fn new()-> Self {
        Broadcaster { outputs: Vec::new() }
    }
}

impl Module for Broadcaster {
    fn transfer(&mut self, input: Vec<bool>) -> Vec<bool> {
        let mut output = Vec::new();
        for _ in self.outputs.iter() {
            output.push(false)
        }
        output
    }
}
