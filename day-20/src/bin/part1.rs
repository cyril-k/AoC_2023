use std::{collections::{HashMap, VecDeque}, fmt::Debug};

fn main () {
    let input = include_str!("./part1.txt");
    let output = part1(input);
    println!("{output}")
}

fn part1(input: &str) -> String {
    let mut modules_map: HashMap<String, Module> = HashMap::new();

    for line in input.lines() {
        let splits: Vec<_> = line.split(" -> ").collect();

        let mut module_name = String::new();
        let output_mod: Module;
        let outputs = splits[1].split(", ").map(|s| s.to_owned()).collect::<Vec<String>>();
        
        if splits[0].starts_with('%') {
            module_name.push_str(&splits[0][1..]);
            output_mod = Module {
                module_type: ModuleType::FlipFlop(false),
                outputs
            };
        } else if splits[0].starts_with('&') {
            let conj_units = HashMap::new();
            module_name.push_str(&splits[0][1..]);
            output_mod = Module {
                module_type: ModuleType::Conjunction(conj_units),
                outputs
            };
        } else {
            module_name.push_str("broadcaster");
            output_mod = Module {
                module_type: ModuleType::Broadcaster,
                outputs
            }
        }
        

        modules_map.insert(module_name, output_mod);
    }

    let mut network = Network::new(modules_map);
    for _ in 0..1000 {
        network.counter_l += 1; // initial button push
        network.bfs();
        network.pulse_sequence.push_back((String::from("broadcaster"), Pulse {state: false, source: String::from("button")}));
    }
    (network.counter_h * network.counter_l).to_string()
    
    // "output".to_string()
}


struct Network {
    module_map: HashMap<String, Module>,
    pulse_sequence: VecDeque<(String, Pulse)>,
    counter_h: usize,
    counter_l: usize,
} 

impl Network {
    fn new(mut module_map:HashMap<String, Module>) -> Self {
        let mut conj_updates = HashMap::new();

        for (name, module) in &module_map {
            if let ModuleType::Conjunction(_) = module.module_type {
                for (name_n, module_n) in &module_map {
                    if module_n.outputs.contains(name) {
                        conj_updates
                            .entry(name.clone())
                            .or_insert_with(HashMap::new)
                            .insert(name_n.clone(), false);
                    }
                }
            }
        }

        for (name, updates) in conj_updates {
            if let Some(module) = module_map.get_mut(&name) {
                if let ModuleType::Conjunction(ref mut conj_units) = module.module_type {
                    *conj_units = updates;
                }
            }
        }

        let mut pulse_sequence = VecDeque::new();
        pulse_sequence.push_back((String::from("broadcaster"), Pulse {state: false, source: String::from("button")}));

        Network {
            module_map,
            pulse_sequence,
            counter_h: 0,
            counter_l: 0,
        }
    }

    fn bfs(&mut self) {
        while let Some((module_name, input_pulse)) = self.pulse_sequence.pop_front() {
            if let Some(module) = self.module_map.get_mut(&module_name) {
                let output_pulses = module.transfer(input_pulse, module_name);
                for (output_module, output_pulse) in output_pulses {
                    if output_pulse.state {
                        self.counter_h += 1;
                    } else {
                        self.counter_l += 1;
                    }

                    self.pulse_sequence.push_back((output_module.clone(), output_pulse));
                }
            }
        }
    }
}

#[derive(Debug)]
struct Pulse {
    state: bool,
    source: String
}

#[derive(Debug)]
enum ModuleType {
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
    Broadcaster,
}

#[derive(Debug)]
struct Module {
    module_type: ModuleType,
    outputs: Vec<String>
}

impl Module {
    fn transfer(&mut self, input_pulse: Pulse, input_name: String) -> Vec<(String, Pulse)> {
        let mut output_seq = Vec::new();

        let output_pulse_state = match self.module_type {
            ModuleType::FlipFlop(ref mut state) => {
                if !input_pulse.state {
                    *state = !*state;
                    Some(*state)
                } else {
                    None
                }
            },
            ModuleType::Conjunction(ref mut unit_states) => {
                unit_states.insert(input_pulse.source, input_pulse.state);
                Some(!unit_states.values().all(|&state| state))
            },
            ModuleType::Broadcaster => Some(input_pulse.state),
            _ => unreachable!(), 
        };
        
        if output_pulse_state.is_some() {
            for output_name in &self.outputs {
                output_seq.push((output_name.clone(), Pulse { state: output_pulse_state.unwrap(), source: input_name.clone() }));
            }
        }

        output_seq
    }
}