use crate::utilities;
use std::collections::HashMap;

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn part1_test1() {
        let result = part1("./input/day8_test1.txt");

        assert_eq!(result, "Path requires 2 steps");
    }

    #[test]
    fn part1_test2() {
        let result = part1("./input/day8_test2.txt");

        assert_eq!(result, "Path requires 6 steps");
    }

    #[test]
    fn part2_test() {
        let result = part2("./input/day8_test3.txt");

        assert_eq!(result, "Path requires 6 steps");
    }
}

pub fn part1(path: &str) -> String {
    let lines = utilities::string_iterator(path);

    let directions = Directions::read(lines);
    let steps = directions.count_steps();
    
    // directions.find_cycle(&['A'; 3]);

    format!("Path requires {steps} steps")
}

pub fn part2(path: &str) -> String {
    let lines = utilities::string_iterator(path);

    let directions = Directions::read(lines);

    // directions.find_all_cycles();

    let steps = directions.count_steps_multistart();

    format!("Path requires {steps} steps")
}

struct Directions {
    turns: Vec<usize>,
    connections: HashMap<[char; 3], [[char; 3]; 2]>
}

impl Directions {
    fn read(mut lines: impl Iterator<Item = String>) -> Self {
        let turns = lines.next().expect("Couldn't get first line")
            .chars()
            .map(|ch| {
                    match ch {
                        'L' => 0,
                        'R' => 1,
                        _ => panic!("Invalid direction")
                    }
                })
            .collect();
        _ = lines.next();

        let mut connections = HashMap::new();
        for line in lines {
            let mut chars = line.chars();
            let origin = Self::next_three_letters(& mut chars);
            let left = Self::next_three_letters(& mut chars);
            let right = Self::next_three_letters(& mut chars);
            connections.insert(origin, [left, right]);
        }
        // println!("{} connections read", connections.len());
        Self {turns, connections}
    }

    fn next_three_letters(chars: & mut impl Iterator <Item = char>) -> [char; 3] {
        let mut letters = [' ', ' ', ' '];
        let mut i: usize = 0;

        while i < 3 {
            let next_char = chars.next().expect("Ran out of characters");
            if next_char.is_uppercase() && next_char.is_ascii_alphabetic() {
                letters[i] = next_char;
                i += 1;
            }
        }

        letters
    }

    fn count_steps(&self) -> u32 {
        let mut count = 0u32;
        let mut moves = self.turns.iter().cycle();
        let mut position = &['A'; 3];
        while *position != ['Z'; 3] {
            let direction: usize = *moves.next().expect("Next move read error");

            position = &self.connections
                .get(position)
                .expect("No matching directions")
                [direction];

            count += 1;
        }

        count
    }

    // fn count_steps_multistart(&self) -> u32 {
    //     let mut steps: u32 = 0;
    //     let mut tracker: u32 = 2u32.pow(10);
    //     let mut moves = self.turns.iter().cycle();
    //     let mut positions: Vec<&[char; 3]> = self.connections
    //         .keys()
    //         .filter(|chs| chs[2] == 'A')
    //         .collect();
    
    //     while positions.iter().any(|chs| chs[2] != 'Z') {
    //         if steps == tracker {
    //             // println!("Reached {tracker}");
    //             tracker *= 2;
    //         }
    //         if steps == u32::MAX {
    //             panic!("Overflowing count");
    //         }
    //         let direction: usize = *moves.next().expect("Next move read error");

    //         for position in &mut positions{
    //             *position = &self.connections
    //                 .get(*position)
    //                 .expect("No matching directions")
    //                 [direction];
    //         }
    //         steps += 1;
    //         // println!("{steps}");
    //     }

    //     steps
    // }

    // I only realized later that the common pattern of 1 iterations, then a cycle in which the target is at the start of the last iteration means that that all paths to the destination are multiples of the cycle length, so finding steps is just a matter of finding the least common multiple.
    fn count_steps_multistart(&self) -> u128 {
        let moves_per_iter = self.turns.len() as u128;
        let mut cycles = self.find_all_cycles();
        // println!("{cycles:?}");

        loop {
            cycles.sort_by_key(|c| c.current_z);

            let new_z = cycles[0].step();
            
            // println!("positions {positions:?}");
            if cycles.iter().all(|c| c.current_z == new_z) {
                break;
            }
        }

        cycles[0].current_z * moves_per_iter
    }


    /// Returns a tuple 3 `u32` values: 
    /// - The number of iterations before the cycle begins
    /// - The number of iterations in the cycle
    /// - The number of iterations **within** the cycle where the `__Z` position appears
    fn find_cycle(&self, start_point: &[char; 3]) -> CycleCounter {
        let mut iter_count: u128 = 0;
        let mut position = start_point.clone();
        let mut iter_starts: Vec<[char;3]> = Vec::new();
        let mut z_point: Option<u128> = None;
        while !iter_starts.contains(&position) {
            iter_starts.push(position);
            if position[2] == 'Z' {
                if z_point.is_none() {
                    z_point = Some(iter_count);
                } else {
                    panic!("Found multiple Z points");
                }
            }
            
            iter_count += 1;

            for direction in &self.turns {
                position = self.connections
                    .get(&position)
                    .expect("No matching directions")
                    [*direction].clone();
            }

        }
        if z_point.is_none() {
            panic!("Found no Z point");
        }
        let cycle_start = position;
        let pre_cycle_iters = iter_starts.iter().position(|a| *a == cycle_start).expect("Couldn't find cycle start point") as u128;
        
        // println!("Found cycle starting with {:?} in iterations {} to {}, with Z-points {:?}", cycle_start, pre_cycle_iters+1, iter_count, z_point);

        CycleCounter::new(
            pre_cycle_iters,
            iter_count - pre_cycle_iters,
            z_point.unwrap() - pre_cycle_iters
        )
    }

    fn find_all_cycles(&self) -> Vec<CycleCounter> {
        let starts: Vec<&[char; 3]> = self.connections
            .keys()
            .filter(|chs| chs[2] == 'A')
            .collect();

        starts
            .iter()
            .map(|start| self.find_cycle(start))
            .collect()
    }
}

#[derive(Debug)]
struct CycleCounter {
    // pre_cycle_iters: u128,
    cycle_length: u128,
    // z_point: u128,
    current_z: u128
}

impl CycleCounter {
    fn new(pre_cycle_iters: u128, cycle_length: u128, z_point: u128) -> Self {
        let current_z = z_point + pre_cycle_iters;
        Self {cycle_length, current_z}
    }
    
    fn step(& mut self) -> u128 {
        self.current_z += self.cycle_length;
        self.current_z
    }
}
