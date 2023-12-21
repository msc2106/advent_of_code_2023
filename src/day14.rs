use crate::utilities;
// use std::collections::HashMap;

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn part1_test1() {
        let result = part1("./input/day14_test1.txt");

        assert_eq!(result, "The total load is 136")
    }

    #[test]
    fn part2_test1() {
        let lines = utilities::string_iterator("./input/day14_test1.txt");
        let mut platform = PlatformFull::read(lines);
        platform.roll_up();
        let total = platform.calculate_load();
        assert_eq!(total, 136);
    }

    // #[test]
    // fn part2_test2() {
    //     let lines = utilities::string_iterator("./input/day14_test1.txt");
    //     let mut platform = PlatformFull::read(lines);
    //     platform.cycle(3);
    //     let answer = ".....#....\n....#...O#\n.....##...\n..O#......\n.....OOO#.\n.O#...O#.#\n....O#...O\n.......OOO\n#...O###.O\n#.OOO#...O\n";
    //     assert_eq!(answer, platform.as_string());
    // }

    #[test]
    fn part2_test3() {
        let result = part2("./input/day14_test1.txt");

        assert_eq!(result, "The total load is 64")
    }
}

pub fn part1(path: &str) -> String {
    let lines = utilities::string_iterator(path);
    let platform = Platform::read(lines);
    let total = platform.calculate_load();

    format!("The total load is {total}")
}

pub fn part2(path: &str) -> String {
    let lines = utilities::string_iterator(path);
    // let mut platform = PlatformFull::read(lines);
    // platform.cycle(1000000000);
    // let total = platform.calculate_load();
    let platform = PlatformFull::read(lines);
    let total = platform.load_after_cycles(1000000000);

    format!("The total load is {total}")
}

struct Platform {
    rocks: Vec<Vec<usize>>,
    stoppers: Vec<Vec<usize>>,
    depth: usize,
}

impl Platform {
    fn read(lines: impl Iterator<Item = String>) -> Self {
        let mut rocks: Vec<Vec<usize>> = Vec::new();
        let mut stoppers: Vec<Vec<usize>> = Vec::new();
        let mut depth: usize = 0;

        for (rownum, line) in lines.into_iter().enumerate() {
            for (colnum, ch) in line.chars().enumerate() {
                if rocks.len() == colnum {
                    rocks.push(Vec::new());
                }
                if stoppers.len() == colnum {
                    stoppers.push(Vec::new());
                }

                if ch == '#' {
                    stoppers[colnum].push(rownum);
                } else if ch == 'O' {
                    rocks[colnum].push(rownum);
                }
            }
            depth = rownum + 1;
        }

        Self {rocks, stoppers, depth}
    }

    fn calculate_load(&self) -> usize {
        let mut total_load: usize = 0;
        for (rock_col, stopper_col) in self.rocks.iter().zip(self.stoppers.iter()) {
            let mut stopper_index: usize = 0;
            let mut packed_rocks: Vec<usize> = Vec::new();
            for rock in rock_col {
                while stopper_index < stopper_col.len() && *rock > stopper_col[stopper_index] {
                    stopper_index += 1;
                }
                let mut open_spot: usize = 0;
                if stopper_index > 0 {
                    open_spot = stopper_col[stopper_index - 1] + 1;
                }
                if let Some(last_rock) = packed_rocks.last() {
                    open_spot = open_spot.max(*last_rock + 1);
                }
                packed_rocks.push(open_spot);
            }
            // println!("Packed rocks: {packed_rocks:?}");
            // println!("Prev load: {total_load}");
            total_load = packed_rocks.into_iter()
                                .fold(
                                    total_load, 
                                    | l, r | {
                                        l + self.depth - r
                                    }
                                );
            // println!("New load: {total_load}");
        }

        total_load
    }
}

#[derive(Clone)]
struct PlatformFull {
    array: Vec<Element>,
    ncols: usize,
    nrows: usize
}

impl PlatformFull {
    fn read(lines: impl Iterator<Item = String>) -> Self {
        let mut array: Vec<Element> = Vec::new();
        let mut ncols: usize = 0;
        let mut nrows: usize = 0;

        for line in lines {
            nrows += 1;
            let mut this_row_len: usize = 0;
            for ch in line.chars() {
                this_row_len += 1;
                array.push(Element::read(&ch));
            }
            ncols = this_row_len;
        }

        Self {array, ncols, nrows}
    }

    fn roll_up(&mut self) {
        let enumerate_rollers = self.array.clone()
            .into_iter()
            .enumerate()
            .filter(| (_, e) | e.is_roller());

        for (i, _) in enumerate_rollers {
            let mut new_pos: usize = i;
            while new_pos >= self.ncols && self.array[new_pos-self.ncols].is_empty() {
                new_pos -= self.ncols;
            }
            self.array[i] = Element::Empty;
            self.array[new_pos] = Element::Roller;
        }
    }

    fn roll_left(&mut self) {
        let enumerate_rollers = self.array.clone()
            .into_iter()
            .enumerate()
            .filter(| (_, e) | e.is_roller());

        for (i, _) in enumerate_rollers {
            let mut new_pos: usize = i;
            while new_pos > i/self.ncols * self.ncols && self.array[new_pos-1].is_empty() {
                new_pos -= 1;
            }
            self.array[i] = Element::Empty;
            self.array[new_pos] = Element::Roller;
        }
    }

    fn roll_down(&mut self) {
        let enumerate_rollers = self.array.clone()
            .into_iter()
            .enumerate()
            .filter(| (_, e) | e.is_roller())
            .rev();

        for (i, _) in enumerate_rollers {
            let mut new_pos: usize = i;
            while new_pos < (self.nrows-1) * self.ncols && self.array[new_pos+self.ncols].is_empty() {
                new_pos += self.ncols;
            }
            self.array[i] = Element::Empty;
            self.array[new_pos] = Element::Roller;
        }
    }

    fn roll_right(&mut self) {
        let enumerate_rollers = self.array.clone()
            .into_iter()
            .enumerate()
            .filter(| (_, e) | e.is_roller())
            .rev();

        for (i, _) in enumerate_rollers {
            let mut new_pos: usize = i;
            while new_pos < (i/self.ncols + 1) * self.ncols - 1 && self.array[new_pos+1].is_empty() {
                new_pos += 1;
            }
            self.array[i] = Element::Empty;
            self.array[new_pos] = Element::Roller;
        }
    }

    fn roller_indices(&self) -> Vec<usize> {
        self.array.iter().enumerate().filter(|(_, e)| e.is_roller()).map(|(i, _)| i).collect()
    }

    // fn memo_string(&self) -> String {
    //     let mut output = String::new();

    //     for index in self.roller_indices() {
    //         output.push_str(&format!("{index},"));
    //     }

    //     output
    // }

    // fn update_from_memo(& mut self, memo_string: &str) {
    //     let new_indices: Vec<usize> = memo_string
    //         .split(',')
    //         .filter_map(| s | {
    //             s.parse::<usize>().ok()
    //         }).collect();
        
    //     for (i, entry) in self.array.iter_mut().enumerate() {
    //         if new_indices.contains(&i) {
    //             *entry = Element::Roller;
    //         } else if entry.is_roller() {
    //             *entry = Element::Empty;
    //         }
    //     }
    // }

    fn load_after_cycles(&self, times:usize) -> usize {
        let mut history: Vec<Vec<usize>> = Vec::new();
        let mut loads: Vec<usize> = Vec::new();
        let mut working_copy = self.clone();
        while !history.contains(&working_copy.roller_indices()) {
            history.push(working_copy.roller_indices());
            loads.push(working_copy.calculate_load());
            working_copy.roll_up();
            working_copy.roll_left();
            working_copy.roll_down();
            working_copy.roll_right();
        }
        let cycles_completed = history.len();
        let cycles_remaining = times - cycles_completed;
        let loop_start = history.iter().position(| v | *v == working_copy.roller_indices()).unwrap();
        let looping_sequence = &loads[loop_start..];
        let remainder = cycles_remaining % looping_sequence.len();
        looping_sequence[remainder]
    }

    // fn cycle(&mut self, times: usize) {
    //     println!("start");   
    //     let mut checkpoint: usize = 1000;
    //     let mut memo: HashMap<String, String> = HashMap::new();
    //     let mut i: usize = 0;

    //     while i < times {
    //         let mut start = self.memo_string();
    //         while let Some(end) = memo.get(&start) {
    //             if i == times - 1 {
    //                 break;
    //             }
    //             start = end.clone();
    //             i += 1;
    //             if i >= checkpoint {
    //                 println!("{}", i);
    //                 checkpoint *= 10;
    //             }
    //         }
    //         self.update_from_memo(&start);
    //         self.roll_up();
    //         self.roll_left();
    //         self.roll_down();
    //         self.roll_right();
    //         let end = self.memo_string();
    //         memo.insert(start, end);
    //         i += 1;
    //         if i >= checkpoint {
    //             println!("{}: {}", i, self.calculate_load());
    //             checkpoint *= 10;
    //         }
    //     }
    // }

    fn calculate_load(&self) -> usize {
        self.array
            .iter()
            .enumerate()
            .filter(| (_, e) | e.is_roller())
            .fold(
                0usize, 
                | total, (index, _) | {
                    total + self.nrows - index/self.ncols
                }
            )
    }

    // fn as_string(&self) -> String {
    //     let mut output = String::new();

    //     for (i, element) in self.array.iter().enumerate() {
    //         output.push (
    //             if element.is_roller() {
    //                 'O'
    //             } else if element.is_empty() {
    //                 '.'
    //             } else {
    //                 '#'
    //             }
    //         );
    //         if (i+1) % self.ncols == 0 {
    //             // println!("{line}");
    //             output.push('\n');
    //         }
    //     }

    //     output
    // }
}

#[derive(Clone, Copy)]
enum Element {
    Empty,
    Stopper,
    Roller
}

impl Element {
    fn read(ch: &char) -> Self {
        match *ch {
            '.' => Self::Empty,
            '#' => Self::Stopper,
            'O' => Self::Roller,
            _ => panic!("Invalid character")
        }
    }

    fn is_roller(&self) -> bool {
        match self {
            Self::Roller => true,
            _ => false
        }
    }

    fn is_empty(&self) -> bool {
        match self {
            Self::Empty => true,
            _ => false
        }
    }
}