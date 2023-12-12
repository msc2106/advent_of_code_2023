use std::fmt::Display;

use crate::utilities;

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn part1_test1() {
        let result = part1("./input/day11_test1.txt");

        assert_eq!(result, "The sum of shortest paths is 374")
    }
}

pub fn part1(path: &str) -> String {
    let lines = utilities::string_iterator(path);
    let universe = Universe::read(lines, 2);

    // println!("{universe}");
    let path_sum = universe.sum_paths();

    format!("The sum of shortest paths is {path_sum}")
}

pub fn part2(path: &str) -> String {
    let lines = utilities::string_iterator(path);
    let universe = Universe::read(lines, 1000000);

    // println!("{universe}");
    let path_sum = universe.sum_paths();

    format!("The sum of shortest paths is {path_sum}")
}

/// `true` indicates a galaxy, `false` empty space
struct Universe{
    grid: Vec<Vec<bool>>,
    col_weights: Vec<u128>,
    row_weights: Vec<u128>
}

impl Universe {
    fn read(lines: impl Iterator<Item = String>, multiple: u128) -> Self {
        let mut grid: Vec<Vec<bool>> = Vec::new();
        let mut row_weights: Vec<u128> = Vec::new();
        let mut col_weights: Vec<u128> = Vec::new();
        
        // read line and note row weights
        for line in lines {
            let converted_line: Vec<bool> = line.chars().map(|ch| ch=='#').collect();
            if converted_line.iter().all(|entry| !*entry) {
                row_weights.push(multiple);
            } else {
                row_weights.push(1);
            }
            grid.push(converted_line);
        }
        
        // find and duplicate empty columns
        for col in 0..grid[0].len() {
            let mut empty = true;
            for row in 0..grid.len() {
                if grid[row][col] {
                    empty = false;
                }
            }
            if empty {
                col_weights.push(multiple);
            } else {
                col_weights.push(1);
            }
        }

        
        Self { grid, col_weights, row_weights }
    }

    fn horiz_dist(&self, start:usize, end:usize) -> u128 {
        let i = start.min(end);
        let j = start.max(end);
        self.col_weights[i..j].iter().sum()
    }

    fn sum_paths(&self) -> u128 {
        // let rows = self.grid.len();
        let cols = self.grid[0].len();
        let mut multiplier = vec![0u128; cols];
        let mut depth = vec![0u128; cols];
        let mut total: u128 = 0;

        for (row, row_weight) in self.grid.iter().zip(self.row_weights.iter()) {
            for i in 0..cols {
                if row[i] {
                    // add distances
                    for j in 0..cols {
                        let distance = self.horiz_dist(i, j);
                        total += depth[j] + distance * multiplier[j];
                    }

                    // add new node
                    multiplier[i] += 1;
                }
            }
            // increment depth by multiplier
            depth = depth.into_iter()
                .zip(multiplier.iter())
                .map(| (depth_val, mult_val) | depth_val + mult_val * row_weight)
                .collect();
        }
        // println!("{multiplier:?} \n {depth:?}");
        total
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fmt_str = String::new();
        for line in &self.grid {
            for entry in line {
                if *entry {
                    fmt_str.push('#');
                } else {
                    fmt_str.push('.');
                }
            }
            fmt_str.push('\n');
        }

        write!(f, "{}", fmt_str)
    }
}