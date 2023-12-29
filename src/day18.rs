use crate::utilities;

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn part1_test1() {
        let result = part1("./input/day18_test1.txt");

        assert_eq!(result, "The area dug is 62")
    }

    #[test]
    fn part1_test2() {
        let lines = utilities::string_iterator("./input/day18_test1.txt");
        let mut outline = OutlineMap::read(lines);
        outline.draw_map();

        let output = outline.str_out();
        assert_eq!(output, "#######\n#.....#\n###...#\n..#...#\n..#...#\n###.###\n#...#..\n##..###\n.#....#\n.######\n")
    }

    #[test]
    fn part2_test1() {
        let result = part2("./input/day18_test1.txt");

        assert_eq!(result, "The area dug is 952408144115")
    }

    #[test]
    fn part2_test2() {
        let result = part2("./input/day18_test2.txt");

        assert_eq!(result, "The area dug is 62")
    }
}

pub fn part1(path: &str) -> String {
    let lines = utilities::string_iterator(path);
    let mut outline = OutlineMap::read(lines);
    outline.draw_map();
    let total = outline.count_interior();
    
    // outline.str_out()
    format!("The area dug is {total}")
}

pub fn part2(path: &str) -> String {
    let lines = utilities::string_iterator(path);
    let mut outline = HexOutlineMap::read(lines);
    outline.construct_map();
    let total = outline.count_interior();
    
    // outline.str_out()
    format!("The area dug is {total}")
}

struct OutlineMap {
    instructions: Vec<Instruction>,
    drawn_map: Vec<Option<usize>>,
    num_cols: usize,
    num_rows: usize,
}

impl OutlineMap {
    fn read(lines: impl Iterator<Item = String>) -> Self {
        let instructions: Vec<Instruction> = lines
            .map(| s | {
                let mut parts = s.split(' ');
                let dir_part = parts.next().unwrap();
                let dist_part = parts.next().unwrap();
                let direction = Direction::read(&dir_part);
                let distance: usize = dist_part.parse().unwrap();
                Instruction::new(direction, distance)
            })
            .collect();
        let drawn_map: Vec<Option<usize>> = Vec::new();
        let num_cols = 0;
        let num_rows = 0;

        Self { instructions, drawn_map, num_cols, num_rows }
    }

    fn draw_map(& mut self) {
        // find dimensions and starting point
        let mut farthest_left = 0;
        let mut farthest_right = 0;
        let mut farthest_down = 0;
        let mut farthest_up = 0;
        let mut current_horiz: i32 = 0;
        let mut current_vert: i32 = 0;

        for instruction in &self.instructions {
            let dir = instruction.direction;
            let steps = instruction.distance;
            match dir {
                Direction::Down => {
                    current_vert += steps as i32;
                    farthest_down = farthest_down.max(relu(current_vert));
                },
                Direction::Up => {
                    current_vert -= steps as i32;
                    farthest_up = farthest_up.max(relu(-current_vert));
                },
                Direction::Left => {
                    current_horiz -= steps as i32;
                    farthest_left = farthest_left.max(relu(-current_horiz));
                },
                Direction::Right => {
                    current_horiz += steps as i32;
                    farthest_right = farthest_right.max(relu(current_horiz));
                },
            }
        }

        // initialize map
        
        let num_cols = farthest_right + farthest_left + 1;
        let num_rows = farthest_down + farthest_up + 1;
        let start_index = farthest_up * num_cols + farthest_left;
        let mut drawn_map = vec![None; num_cols * num_rows];
        
        drawn_map[start_index as usize] = Some(0);
        
        // fill in outlines
        let mut current_index = start_index;
        for (i, instruction) in self.instructions.iter().enumerate() {
            let dir = instruction.direction;
            let steps = instruction.distance;
            match dir {
                Direction::Down => {
                    for _ in 0..steps {
                        current_index += num_cols;
                        drawn_map[current_index] = Some(i);
                    }
                },
                Direction::Up => {
                    for _ in 0..steps {
                        current_index -= num_cols;
                        drawn_map[current_index] = Some(i);
                    }
                },
                Direction::Left => {
                    for _ in 0..steps {
                        current_index -= 1;
                        drawn_map[current_index] = Some(i);
                    }
                },
                Direction::Right => {
                    for _ in 0..steps {
                        current_index += 1;
                        drawn_map[current_index] = Some(i);
                    }
                },
            }
        }

        self.drawn_map.append(& mut drawn_map);
        self.num_cols = num_cols;
        self.num_rows = num_rows;
        // println!("{}", self.str_out());

    }

    #[allow(dead_code)]
    fn str_out(& self) -> String {
        let mut out = String::new();
        for i in 0..self.num_rows {
            let row_start = i * self.num_cols;
            let row_end = (i+1) * self.num_cols;
            let line_chars = self.drawn_map[row_start..row_end]
                .iter()
                .map(| b | if b.is_some() {'#'} else {'.'} );
            for ch in line_chars {
                out.push(ch);
            }
            out.push('\n');
        }

        out
    }

    fn adjacent_edges(& self, one: usize, other: usize) -> bool {
        one.abs_diff(other) <= 1 ||
        one.abs_diff(other) == self.instructions.len() - 1
    }

    fn count_interior(& self) -> usize {
        let mut total: usize = 0;
        let mut last_row: Vec<Option<usize>> = vec![None; self.num_cols];
        
        for i in 0..self.num_rows {
            // let mut line = String::new();
            let row_start = i * self.num_cols;
            let row_end = (i+1) * self.num_cols;
            let row = &self.drawn_map[row_start..row_end];
            let mut connected_above = false;
            let mut on_horizontal = false;
            let mut edges_found = 0;
            let mut prev_square: Option<usize> = None;
            let mut prev_square_above: Option<usize> = None;
            let two_rows = row.iter().zip(last_row.iter());
            for (square, square_above) in two_rows {
                if let Some(edge_index) = *square {
                    // line.push('#');
                    total += 1;
                    if let Some(prev_edge) = prev_square {
                        if self.adjacent_edges(prev_edge, edge_index) {
                            on_horizontal = true;
                        } else {
                            edges_found += 1;
                            on_horizontal = false;
                        }
                    } else {
                        edges_found += 1;
                        on_horizontal = false;
                    }

                    if !on_horizontal {
                        if let Some(edge_above) = *square_above {
                            if self.adjacent_edges(edge_index, edge_above) {
                                connected_above = true;
                            } else {
                                connected_above = false;
                            }
                        } else {
                            connected_above = false;
                        }
                    }
                } else {
                    if on_horizontal {
                        if let (Some(prev_edge), Some(prev_edge_above)) = (prev_square, prev_square_above) {
                            if self.adjacent_edges(prev_edge, prev_edge_above) && connected_above {
                                edges_found += 1;
                            }
                        } else if prev_square_above.is_none() && !connected_above {
                            edges_found += 1;
                        }
                        on_horizontal = false;
                    }
                    
                    if edges_found % 2 == 1 {
                        total += 1;
                        // line.push('#');
                    } else {
                        // line.push('.');
                    }
                }

                prev_square = square.clone();
                prev_square_above = square_above.clone();
            }
            last_row.clone_from_slice(row);
            // println!("{line}");
        }

        total
    }
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    direction: Direction,
    distance: usize
}

impl Instruction {
    fn new(direction: Direction, distance: usize) -> Self {
        Self {
            direction,
            distance
        }
    }

    fn read_hex(hex_code: &str) -> Self {
        let mut hex_chars: Vec<char> = hex_code.chars().filter(| ch | ch.is_digit(16)).collect();
        let dir_char = hex_chars.pop().unwrap();
        let direction = Direction::read_hex(dir_char);
        let distance = hex_chars.into_iter()
            .filter_map(| ch | ch.to_digit(16))
            .fold(0usize, |l, r | l*16 + r as usize);

        Self { direction, distance }
        
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn read(s: &str) -> Self {
        match s {
            "U" => Self::Up,
            "D" => Self::Down,
            "R" => Self::Right,
            "L" => Self::Left,
            _ => panic!("Invalid direction character")
        }
    }

    fn read_hex(ch: char) -> Self {
        match ch {
            '0' => Self::Right,
            '1' => Self::Down,
            '2' => Self::Left,
            '3' => Self::Up,
            _ => {
                println!("{ch}");
                panic!("Invalid value for hex conversion")
            }
        }
    }
}

fn relu(x: i32) -> usize {
    if x < 0 {
        0usize
    } else {
        x as usize
    }
}

struct HexOutlineMap {
    instructions: Vec<Instruction>,
    vert_lines: Vec<VertSpan>,
    horiz_lines: Vec<HorizSpan>
}

impl HexOutlineMap {
    fn read(lines: impl Iterator<Item = String>) -> Self {
        let instructions: Vec<Instruction> = lines
            .map(| s | {
                let parts = s.split('#');
                let code = parts.last().unwrap();
                Instruction::read_hex(code)
            })
            .collect();
        let vert_lines = Vec::new();
        let horiz_lines = Vec::new();
        // println!("{:?}", instructions);
        Self { instructions, vert_lines, horiz_lines }
    }

    fn construct_map(& mut self) {
        let mut prev_direction = self.instructions.last().unwrap().direction;
        let mut current_x: i64 = 0;
        let mut current_y: i64 = 0;
        let mut instructions_copy = self.instructions.clone();
        instructions_copy.push(self.instructions[0]);
        for i in 0..self.instructions.len() {
            let direction = instructions_copy[i].direction;
            let distance = instructions_copy[i].distance as i64;
            match direction {
                Direction::Down => {
                    let span = VertSpan{
                        top: current_y - 1,
                        bottom: current_y - distance + 1,
                        x_pos: current_x
                    };
                    self.vert_lines.push(span);
                    current_y -= distance;
                },
                Direction::Up => {
                    let span = VertSpan{
                        top: current_y + distance - 1,
                        bottom: current_y + 1,
                        x_pos: current_x
                    };
                    self.vert_lines.push(span);
                    current_y += distance;
                },
                Direction::Left => {
                    let right_connection = if prev_direction == Direction::Up {
                        Connector::Down(current_x)
                    } else {
                        Connector::Up(current_x)
                    };
                    let next_direction = instructions_copy[i+1].direction;
                    let end_point = current_x - distance;
                    let left_connection = if next_direction == Direction::Up {
                        Connector::Up(end_point)
                    } else {
                        Connector::Down(end_point)
                    };
                    let span = HorizSpan {
                        left: left_connection,
                        right: right_connection,
                        y_pos: current_y
                    };
                    self.horiz_lines.push(span);
                    current_x = end_point;
                },
                Direction::Right => {
                    let left_connection = if prev_direction == Direction::Up {
                        Connector::Down(current_x)
                    } else {
                        Connector::Up(current_x)
                    };
                    let next_direction = instructions_copy[i+1].direction;
                    let end_point = current_x + distance;
                    let right_connection = if next_direction == Direction::Up {
                        Connector::Up(end_point)
                    } else {
                        Connector::Down(end_point)
                    };
                    let span = HorizSpan {
                        left: left_connection,
                        right: right_connection,
                        y_pos: current_y
                    };
                    self.horiz_lines.push(span);
                    current_x = end_point;
                },
            }
            prev_direction = direction;
        }
        // println!("{:?}", self.horiz_lines);
        // println!("{:?}", self.vert_lines);
        self.vert_lines.sort();
        self.horiz_lines.sort_by_key(| span | {
            (span.y_pos, span.left.position(), span.right.position())
            }
        );
    }

    fn count_interior(& self) -> u64 {
        let mut total: u64 = 0;
        let mut index = 0usize;
        while index < self.horiz_lines.len() {
            let mut row_total: u64 = 0;
            let rownum = self.horiz_lines[index].y_pos;
            // println!("Row: {rownum}");
            let horiz_lines = self.horiz_lines[index..]
                .iter()
                .take_while(| span | {
                    span.y_pos == rownum
                });
            index += horiz_lines.clone().count();
            let vert_lines = self.vert_lines
                .iter()
                .filter(| v | v.contains(rownum));
            let mut spans: Vec<Box<dyn Span>> = Vec::new();
            for span in horiz_lines {
                spans.push(Box::new(span.clone()));
            }
            for span in vert_lines {
                spans.push(Box::new(span.clone()));
            }
            spans.sort_by_key(| span | span.first_edge());
            let mut edges_found = 0;
            let mut prev_edge: Option<i64> = None;
            for span in spans {
                row_total += span.square_count();
                if edges_found % 2 == 1 {
                    row_total += span.first_edge().abs_diff(prev_edge.unwrap()+1) as u64
                }
                edges_found += span.edge_count();
                prev_edge = Some(span.last_edge());
                // println!("Prev edge: {:?}, first edge: {}, last_edge: {}, row total: {}", prev_edge, span.first_edge(), span.last_edge(), row_total);
            }
            // println!("Found on horizontal: {row_total}");
            total += row_total;
            // println!("Cumulative: {total}");

            // reset
            row_total = 0;

            // Consider rows with only vertical lines
            if let Some(next_span) = self.horiz_lines.get(index) {
                let height: u64 = next_span.y_pos.abs_diff(rownum + 1).into();
                if height > 0 {
                    let mut edges_found = 0;
                    let mut prev_edge: Option<i64> = None;
                    let vert_lines = self.vert_lines
                        .iter()
                        .filter(| v | v.contains(rownum+1));
                    for span in vert_lines {
                        row_total += span.square_count() * height;
                        if edges_found % 2 == 1 {
                            row_total += span.first_edge().abs_diff(prev_edge.unwrap()+1) as u64 * height
                        }
                        edges_found += span.edge_count();
                        prev_edge = Some(span.last_edge());
                    }
                }
            }
            // println!("Found on verticals: {row_total}");
            total += row_total;
            // println!("Cumulative: {total}");
        }


        total
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct VertSpan {
    x_pos: i64,
    bottom: i64,
    top: i64,
}

impl VertSpan {
    fn contains(& self, y_pos: i64) -> bool {
        y_pos >= self.bottom &&
        y_pos <= self.top
    }
}

impl Span for VertSpan {
    fn edge_count(& self) -> i64 {
        1
    }

    fn square_count(& self) -> u64 {
        1
    }

    fn first_edge(& self) -> i64 {
        self.x_pos
    }

    fn last_edge(& self) -> i64 {
        self.x_pos
    }
}

#[derive(Clone, Debug)]
struct HorizSpan {
    left: Connector,
    right: Connector,
    y_pos: i64
}

impl Span for HorizSpan {
    fn edge_count(& self) -> i64 {
        if self.left.same_side(&self.right) {
            2
        } else {
            1
        }
    }

    fn square_count(& self) -> u64 {
        self.right.position().abs_diff(self.left.position() - 1)
            .into()
    }
    
    fn first_edge(& self) -> i64 {
        self.left.position()
    }

    fn last_edge(& self) -> i64 {
        self.right.position()
    }
}

#[derive(Clone, Debug)]
enum Connector {
    Down(i64),
    Up(i64)
}

impl Connector {
    fn position(& self) -> i64 {
        match self {
            Self::Down(pos) => *pos,
            Self::Up(pos) => *pos
        }
    }

    fn same_side(& self, other: &Self) -> bool {
        match *self {
            Self::Down(_) => {
                match *other {
                    Self::Down(_) => true,
                    Self::Up(_) => false,
                }
            },
            Self::Up(_) => {
                match *other {
                    Self::Down(_) => false,
                    Self::Up(_) => true
                }
            }
        }
    }
}

trait Span {
    fn edge_count(& self) -> i64;
    fn square_count(& self) -> u64;
    fn last_edge(& self) -> i64;
    fn first_edge(& self) -> i64;
}
