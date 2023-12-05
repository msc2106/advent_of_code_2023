use crate::utilities;

#[cfg(test)]
mod testing {
    use super::*;
    
    #[test]
    fn test_data_1() {
        let output = part1("./input/day3_test1.txt");
        assert_eq!(output, "The sum of parts values is 4361");
    }

    #[test]
    fn test_data_2() {
        let output = part2("./input/day3_test2.txt");
        assert_eq!(output, "The sum of gear values is 467835");
    }
}

pub fn part1(path: &str) -> String {
    let lines = 
        utilities::lines_from_file(path)
            .filter_map(|s| s.ok());
    let mut schematic = Grid::new();
    schematic.read_lines(lines);
    let parts_nums = schematic.find_parts();
    // println!("{parts_nums:#?}");
    let parts_sum:u32 = parts_nums.into_iter().sum();
    format!("The sum of parts values is {parts_sum}")
}

pub fn part2(path: &str) -> String {
    let lines = 
        utilities::lines_from_file(path)
            .filter_map(|s| s.ok());
    let mut schematic = Grid::new();
    schematic.read_lines(lines);
    let gears = schematic.find_gears();
    // println!("{gears:#?}");
    let gears_sum:u32 = gears.into_iter().sum();
    format!("The sum of gear values is {gears_sum}")
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Position {
    line: usize,
    cols: (usize, usize)
}

impl Position {
    fn iter_adjacent(&self) -> impl Iterator<Item = (usize, usize)> {
        let mut adjacent_idx:Vec<(i32, i32)> = Vec::new();
        let row = self.line as i32;
        let start_col = self.cols.0 as i32;
        let end_col = self.cols.1 as i32;

        for col in start_col-1 ..= end_col+1 {
            adjacent_idx.push((row-1, col));
        }
        adjacent_idx.push((row, start_col-1));
        adjacent_idx.push((row, end_col+1));
        for col in start_col-1 ..= end_col+1 {
            adjacent_idx.push((row+1, col));
        }

        adjacent_idx.into_iter().filter(|(row, col)| {*row >= 0 && *col >= 0}).map(|(row, col)| {(row as usize, col as usize)})
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Element {
    Empty(),
    Symbol { pos:Position, ch:char },
    Number { pos:Position, num:u32 }
}

impl Element {
    /// Returns an option: `Some(value)` for a number, `None` otherwise
    fn get_number(&self) -> Option<(Position, u32)> {
        if let Element::Number {pos, num} = self {
            Some((*pos, *num))
        } else {
            None
        }
    }

    /// Returns an option: `Some(pos)` for a '*', `None` otherwise.
    fn get_star(&self) -> Option<Position> {
        if let Element::Symbol {pos, ch:'*'} = self {
            Some(*pos)
        } else {
            None
        }
    }

    // fn is_symbol(&self) -> bool {
    //     if let Element::Symbol{pos:_, ch:_} = self {
    //         true
    //     } else {
    //         false
    //     }
    // }
}

struct Grid {
    num_rows: usize,
    num_cols: usize,
    elements: Vec<Element>,
    grid: Vec<usize>
}

impl Grid {
    fn new() -> Self {
        let elements = vec![Element::Empty()];
        Grid { num_rows: 0, num_cols: 0, elements, grid: Vec::new() }
    }

    fn read_lines(& mut self, lines: impl Iterator<Item=String>) {
        for (rownum, line) in lines.enumerate() {
            self.num_rows += 1;
            let mut len = 0usize;
            let mut current_num:Option<(Position, u32)> = None;
            for (colnum, ch) in line.chars().enumerate() {
                len += 1;
                if let Some(num) = ch.to_digit(10) {
                    if let Some((pos, prev_num)) = current_num {
                        let new_pos = Position { line: rownum, cols: (pos.cols.0, colnum) };
                        current_num = Some((new_pos, prev_num*10 + num));
                    } else {
                        let pos = Position { line:rownum, cols:(colnum, colnum)};
                        current_num = Some((pos, num));
                    }
                    self.grid.push(self.elements.len())
                } else {
                    if let Some((pos, num)) = current_num {
                        self.elements.push(Element::Number{pos, num});
                        current_num = None;
                    }
                    
                    if ch == '.' {
                        self.grid.push(0);
                    } else {
                        self.grid.push(self.elements.len());
                        let pos = Position {line:rownum, cols:(colnum, colnum)};
                        self.elements.push(Element::Symbol{pos, ch});
                    }
                }
                
            }
            if let Some((pos, num)) = current_num {
                self.elements.push(Element::Number{pos, num});
            }
            
            if self.num_cols == 0 {
                self.num_cols = len;
            } else if self.num_cols != len {
                println!("Prev len: {}, current len {}", self.num_cols, len);
                println!("{}", line);
                panic!("Found lines of non-equal length");
            }
        }
    }

    fn find_parts(&self) -> Vec<u32> {
        let mut part_nums = Vec::new();
        let pos_num = self.elements.iter().filter_map(|e| e.get_number());
        for (pos, num) in pos_num {
            let adjacent = pos.iter_adjacent();
            for (row, col) in adjacent {
                if let Some(Element::Symbol { pos: _, ch: _ }) = self.get(row, col) {
                    part_nums.push(num);
                    break;
                }
            }
        }
        part_nums
    }

    fn find_gears(&self) -> Vec<u32> {
        let mut gears: Vec<u32> = Vec::new();
        let positions = self.elements.iter().filter_map(|e| e.get_star());
        for pos in positions {
            // println!("* {pos:?}");
            let adjacent = pos.iter_adjacent();
            let mut found_elems:Vec<Element> = Vec::new();
            for (row, col) in adjacent {
                // println!("adj: {row}, {col}");
                if let Some(elem) = self.get(row, col) {
                    // println!("Found: {elem:?}");
                    if !found_elems.contains(&elem) {
                        found_elems.push(elem);
                    }
                }
            }
            let found_nums = 
                found_elems
                    .iter()
                    .filter_map(|e| {e.get_number()})
                    .map(|(_, v)| v);
            if found_nums.clone().count() == 2 {
                gears.push(found_nums.product());
            }
        }
        gears
    }

    fn get(&self, row:usize, col:usize) -> Option<Element> {
        if row >= self.num_rows || col >= self.num_cols {
            None
        } else {
            let idx = self.grid[row*self.num_cols + col];
            Some(self.elements[idx])
        }
    }
}


