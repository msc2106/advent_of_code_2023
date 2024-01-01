use crate::utilities;
use std::collections::{HashMap, VecDeque};
use std::iter;

#[cfg(test)]
mod testing{
    use super::*;

    #[test]
    fn part1_test1() {
        let result = part1("./input/day20_test1.txt");

        assert_eq!(result, "The product is 32000000")
    }

    #[test]
    fn part1_test2() {
        let result = part1("./input/day20_test2.txt");

        assert_eq!(result, "The product is 11687500")
    }
}


pub fn part1(path: &str) -> String {
    let lines = utilities::string_iterator(path);
    let mut device = CommDevice::read(lines);
    let total = device.propagate_n(1000);

    format!("The product is {total}")
}

pub fn part2(path: &str) -> String {
    let lines = utilities::string_iterator(path);
    let mut device = CommDevice::read(lines);
    let count = device.count_to_rx();

    format!("Required {count} button presses")
}

struct CommDevice {
    modules: HashMap<String, Module>,
}

impl CommDevice {
    fn read(lines: impl Iterator<Item = String>) -> Self {
        let mut modules = HashMap::new();
        let mut conjunctions: Vec<String> = Vec::new();

        let key_val = lines
            .map(| s | Module::read(s));

        for (key, val) in key_val {
            if val.module_type.is_conjunction() {
                conjunctions.push(key.clone());
            }
            modules.insert(key, val);
        }

        // initialize conjuctions
        let conjunction_pairs: Vec<(String, String)> = modules
            .iter()
            .flat_map(
                | (source, module) | {
                    iter::repeat(source.clone())
                        .zip(module.destinations.clone().into_iter())
                        .filter(| (_, destination) | conjunctions.contains(destination))
                }
            )
            .collect();
        for (source, destination) in conjunction_pairs {
            let conjunction = modules.get_mut(&destination).unwrap();
            conjunction.module_type.add_conjunction(source);
        }

        // println!("{modules:?}");

        Self { modules }
    }
    
    fn propagate_n(&mut self, n: usize) -> u32 {
        let mut low_count: u32 = 0;
        let mut high_count: u32 = 0;
        for _ in 0..n {
            let (low, high) = self.push_button();
            low_count += low;
            high_count += high;
        }

        low_count * high_count
    }

    fn push_button(&mut self) -> (u32, u32) {
        let mut low_count: u32 = 0;
        let mut high_count: u32 = 0;
        let mut stack = VecDeque::new();
        stack.push_back((String::from(""), String::from("broadcaster"), Pulse::Low));

        while let Some((source, destination, pulse)) = stack.pop_front() {
            // println!("Pulse: {pulse:?} to {destination}");
            if pulse.is_high() {
                high_count += 1;
            } else {
                low_count += 1;
            }
            // println!("{destination}");
            if let Some(module) = self.modules.get_mut(&destination) {
                if let Some((pulse, new_destinations)) = module.process(pulse, &source) {
                    for new_destination in new_destinations {
                        // println!("Sending {pulse:?} from {destination} to {new_destination}");
                        stack.push_back((destination.clone(), new_destination, pulse));
                    }
                }
            }
        }

        (low_count, high_count)
    }

    // By looking at the patters in the conjunction that links to the final module
    // it was clear that there were different periodic patterns for high pulses to each of the 
    // 4 modules that linked to it. This code just finds the first high pulse for each and multiplies them
    // to find the LCM.
    fn count_to_rx(&mut self) -> u64 {
        let mut count: u64 = 0;
        let targets = ["gc", "sz", "xf", "cm"];
        let mut count_to_high: HashMap<String, u64> = HashMap::new();

        loop {
            count += 1;
            let mut stack = VecDeque::new();
            stack.push_back((String::from(""), String::from("broadcaster"), Pulse::Low));
            // let mut pulses_to_rx: Vec<Pulse> = Vec::new();

            while let Some((source, destination, pulse)) = stack.pop_front() {
                // println!("Pulse: {pulse:?} to {destination}");
                // println!("{destination}");
                if let Some(module) = self.modules.get_mut(&destination) {
                    if destination == "zr" && pulse.is_high() && !count_to_high.contains_key(&source) {
                        count_to_high.insert(source.clone(), count);
                    }
                    if let Some((pulse, new_destinations)) = module.process(pulse, &source) {
                        for new_destination in new_destinations {
                            // println!("Sending {pulse:?} from {destination} to {new_destination}");
                            stack.push_back((destination.clone(), new_destination, pulse));
                        }
                    }
                // } else if destination == "rx" {
                    // pulses_to_rx.push(pulse);
                // }
            }
            // println!("{count}: {tracking_zr}");
            // if pulses_to_rx.iter().filter(| pulse | pulse.is_low()).count() == 1 {
                // break;
            }

            if targets.iter().filter(| name | count_to_high.contains_key(**name)).count() == 4 {
                break;
            }
        }

        count_to_high.values().product()
    }
}

#[derive(Debug)]
struct Module {
    module_type: ModuleType,
    destinations: Vec<String>
}

impl Module {
    fn read(line: String) -> (String, Self) {
        let mut parts = line.split(" -> ");
        let module_part = parts.next().unwrap();

        let dest_part = parts.next().unwrap();

        let destinations = dest_part
            .split(", ")
            .map(| s | String::from(s))
            .collect();

        let (name, module_type) = if module_part == "broadcaster" {
                (String::from("broadcaster"), ModuleType::Broadcast)
            } else if let Some(name) = module_part.strip_prefix('%') {
                (String::from(name), ModuleType::new_switch())
            } else if let Some(name) = module_part.strip_prefix('&') {
                (String::from(name), ModuleType::new_conjunction())
            } else {
                panic!("Couldn't read module type and name");
            };

        let new_module = Module { module_type, destinations };
        (name, new_module)
    }
    
    fn process(& mut self, pulse: Pulse, source: &str) -> Option<(Pulse, Vec<String>)> {
        let output_pulse = match self.module_type {
            ModuleType::Broadcast => Pulse::Low,
            ModuleType::Switch(_) => {
                if pulse.is_low() {
                    self.module_type.switch_state()
                } else {
                    return None;
                }
            },
            ModuleType::Conjunction(_) => {
                self.module_type.remember_pulse(source, pulse)
            }
        };

        Some((output_pulse, self.destinations.clone()))
    }
}

#[derive(Debug)]
enum ModuleType {
    Broadcast,
    Switch(bool),
    Conjunction(HashMap<String, Pulse>),
}

impl ModuleType {
    fn new_switch() -> Self {
        Self::Switch(false)
    }

    fn is_conjunction(&self) -> bool {
        match self {
            Self::Conjunction(_) => true,
            _ => false
        }
    }

    fn new_conjunction() -> Self {
        let map = HashMap::new();
        Self::Conjunction(map)
    }

    fn add_conjunction(&mut self, name: String) {
        if let Self::Conjunction(destinations) = self {
            destinations.insert(name, Pulse::Low);
        }
    }

    fn switch_state(&mut self) -> Pulse {
        if let Self::Switch(state) = self {
            *state = !*state;
            if *state {
                Pulse::High
            } else {
                Pulse::Low
            }
        } else {
            panic!("Tried to switch state on non-switch")
        }
    }

    fn remember_pulse(&mut self, source: &str, pulse: Pulse) -> Pulse {
        if let Self::Conjunction(destinations) = self {
            destinations.insert(String::from(source), pulse);
            if destinations.values().all(| memory | memory.is_high()) {
                Pulse::Low
            } else {
                Pulse::High
            }
        } else {
            panic!("Tried to remember pulse on non-conjunction")
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Pulse {
    High,
    Low
}

impl Pulse {
    fn is_high(&self) -> bool {
        match self {
            Pulse::High => true,
            Pulse::Low => false           
        }
    }

    fn is_low(&self) -> bool {
        !self.is_high()
    }
}