use crate::utilities;
use std::ops::Range;
// use std::collections:HashMap;

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn part1_test1() {
        let lines = utilities::string_iterator("./input/day21_test1.txt");
        let stepper = Stepper::read(lines);
        let total = stepper.count_destinations(6);
        assert_eq!(total, 16);
    }
}

pub fn part1(path: &str) -> String {
    let lines = utilities::string_iterator(path);
    let stepper = Stepper::read(lines);
    let total = stepper.count_destinations(64);
    format!("He can reach {total} garden plots")
}

struct Stepper {
    adjacency: Vec<Vec<usize>>,
    start: usize
}

impl Stepper {
    fn read(mut lines: impl Iterator<Item = String>) -> Self {
        let first_line = lines.next().unwrap();
            
        let first_row = first_line.chars().map(| ch | Square::read(ch));
        let mut square_types = Chart::new(first_row);
        for line in lines {
            let new_row = line.chars().map(| ch | Square::read(ch));
            square_types.append_row(new_row);
        }

        let start = square_types.find_with(| square | square.is_start());

        let adjacency: Vec<Vec<usize>> = square_types
            .range()
            .map(
                | i | {
                    square_types.neigbors(i)
                        .into_iter()
                        .filter(| i | square_types.entries[*i].is_plot())
                        .collect::<Vec<usize>>()
                }
            )
            .collect();
        // println!("{adjacency:?}");

        Self {adjacency, start}
    }

    fn count_destinations(&self, max_steps: usize) -> usize {
        let mut visited_even: Vec<bool> = vec![false; self.adjacency.len()];
        let mut visited_odd: Vec<bool> = vec![false; self.adjacency.len()];
        let mut priority: Vec<usize> = vec![max_steps; self.adjacency.len()];
        priority[self.start] = 0;
        let min_priority = | priority_vec: &Vec<usize> | {
            let (index, val) = priority_vec
                .iter()
                .enumerate()
                .min_by_key(| (_, priority) | **priority)
                .unwrap();
            if *val < max_steps {
                Some((index, *val))
            } else {
                None
            }
        };
        visited_even[self.start] = true;
        while let Some((index, steps_taken)) = min_priority(&priority) {
            // println!("{index}: {steps_taken}");
            priority[index] = max_steps;
            let neighbors = &self.adjacency[index];
            let next_step_is_even = (steps_taken+1) % 2 == 0;
            for neighbor in neighbors.iter().map(|index| *index) {
                if !visited_even[neighbor] && !visited_odd[neighbor] {
                    priority[neighbor] = priority[neighbor].min(steps_taken + 1);
                    if next_step_is_even {
                        visited_even[neighbor] = true;
                    } else {
                        visited_odd[neighbor] = true;
                    }
                }
            }
        }
        // println!("Even: {visited_even:?}");
        // println!("Odd: {visited_odd:?}");
        visited_even.into_iter().filter(| visited | *visited).count()
    }

    // fn walk_recur(&self, position: usize, step_number: usize, memo: &mut HashMap<usize, [Option<u64>;64]>) -> u64 {
    //     // println!("{memo:?}");
    //     if step_number == 64 {
    //         return 1;
    //     }
    //     if !memo.contains_key(&position) {
    //         memo.insert(position, [None; 64]);
    //     }
    //     if let Some(count) = memo[&position][step_number] {
    //         return count;
    //     }
    //     let neighbors = &self.adjacency[position];
    //     let count = neighbors.iter()
    //         .map(| neighbor | self.walk_recur(*neighbor, step_number+1, memo))
    //         .sum();
    //     memo.get_mut(&position).unwrap()[step_number] = Some(count);
    //     count
    // }
}

struct Chart<T> {
    entries: Vec<T>,
    numrows: usize,
    numcols: usize
}

impl <T> Chart<T> {
    fn new(first_row: impl Iterator<Item = T>) -> Self {
        let entries: Vec<T> = first_row.collect();
        let numrows = 1;
        let numcols = entries.len();
        Self { entries, numrows, numcols }
    }

    fn append_row(&mut self, new_row: impl Iterator<Item = T>) {
        self.entries.extend(new_row);
        self.numrows += 1;
        if self.entries.len() != self.numcols * self.numrows {
            panic!("New row was wrong length");
        }
    }

    // fn update(&mut self, index: usize, new_data: T) {
    //     self.entries[index] = new_data;
    // }

    // fn get(&self, index: usize) -> &T {
    //     &self.entries[index]
    // }

    fn range(&self) -> Range<usize> {
        0..self.entries.len()
    }
    
    fn neigbors(&self, index: usize) -> Vec<usize> {
        let col = index % self.numcols;
        let row = index / self.numcols;

        (0..self.entries.len())
            .filter(
                | i | {
                    let neighbor_col = i % self.numcols;
                    let neighbor_row = i / self.numcols;
                    (col == neighbor_col && row.abs_diff(neighbor_row) == 1) || 
                    (row == neighbor_row && col.abs_diff(neighbor_col) == 1)
                }
            )
            .collect()
    }

    fn find_with(&self, test: impl Fn(&T) -> bool) -> usize {
        self.entries
            .iter()
            .position(test)
            .unwrap()
    }
}

enum Square {
    Plot,
    Rock,
    Start
}

impl Square {
    fn read(ch: char) -> Self {
        if ch == '#' {
            Self::Rock
        } else if ch == 'S' {
            Self::Start
        } else {
            Self::Plot
        }
    }

    fn is_plot(&self) -> bool {
        match self {
            Self::Plot | Self::Start => true,
            Self::Rock => false
        }
    }

    fn is_start(&self) -> bool {
        match self {
            Self::Start => true,
            _ => false
        }
    }
}