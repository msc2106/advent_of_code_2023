use crate::utilities;
use std::{ops::Range, collections::HashMap};

pub fn part1(path: &str) -> String {
    let lines = utilities::string_iterator(path);
    let stepper = Stepper::read(lines);
    let total = stepper.count_destinations();
    format!("He can reach {total} garden plots")
}

struct Stepper {
    adjacency: Vec<Vec<usize>>,
    start: usize
}

impl Stepper {
    fn read(mut lines: impl Iterator<Item = String>) -> Self {
        let first_line = lines.next().unwrap();
        let find_start = | cloned_line: String | {
            cloned_line
                .chars()
                .enumerate()
                .find_map(
                    | (i, ch) | {
                        if ch == 'S' {
                            Some(i)
                        } else {
                            None
                        }
                    }
                )
        };
        let mut start = find_start(first_line.clone());
            
        let first_row = first_line.chars().map(| ch | Square::read(ch));
        let mut square_types = Chart::new(first_row);
        for line in lines {
            if start.is_none() {
                start = find_start(line.clone());
            }
            let new_row = line.chars().map(| ch | Square::read(ch));
            square_types.append_row(new_row);
        }

        let start = start.unwrap();

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
        println!("{adjacency:?}");

        Self {adjacency, start}
    }

    fn count_destinations(&self) -> u64 {
        let mut memo: HashMap<usize, [Option<u64>;64]> = HashMap::new();
        
        self.walk_recur(self.start, 0, &mut memo)
    }

    fn walk_recur(&self, position: usize, step_number: usize, memo: &mut HashMap<usize, [Option<u64>;64]>) -> u64 {
        // println!("{memo:?}");
        if step_number == 64 {
            return 1;
        }
        if !memo.contains_key(&position) {
            memo.insert(position, [None; 64]);
        }
        if let Some(count) = memo[&position][step_number] {
            return count;
        }
        let neighbors = &self.adjacency[position];
        let count = neighbors.iter()
            .map(| neighbor | self.walk_recur(*neighbor, step_number+1, memo))
            .sum();
        memo.get_mut(&position).unwrap()[step_number] = Some(count);
        count
    }
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
}

enum Square {
    Plot,
    Rock
}

impl Square {
    fn read(ch: char) -> Self {
        if ch == '#' {
            Self::Rock
        } else {
            Self::Plot
        }
    }

    fn is_plot(&self) -> bool {
        match self {
            Self::Plot => true,
            Self::Rock => false
        }
    }
}