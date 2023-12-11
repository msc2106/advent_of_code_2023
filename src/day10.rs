// use std::collections::HashSet;

use crate::utilities;

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn part1_test1() {
        let result = part1("./input/day10_test1.txt");
        assert_eq!(result, "The most distance point is 4 steps")
    }

    #[test]
    fn part1_test2() {
        let result = part1("./input/day10_test2.txt");
        assert_eq!(result, "The most distance point is 8 steps")
    }

    #[test]
    fn part2_test1() {
        let result = part2("./input/day10_test1.txt");
        assert_eq!(result, "There are 1 enclosed squares")
    }

    #[test]
    fn part2_test2() {
        let result = part2("./input/day10_test2.txt");
        assert_eq!(result, "There are 1 enclosed squares")
    }

    #[test]
    fn part2_test3() {
        let result = part2("./input/day10_test3.txt");
        assert_eq!(result, "There are 4 enclosed squares")
    }

    #[test]
    fn part2_test4() {
        let result = part2("./input/day10_test4.txt");
        assert_eq!(result, "There are 8 enclosed squares")
    }

    #[test]
    fn part2_test5() {
        let result = part2("./input/day10_test5.txt");
        assert_eq!(result, "There are 10 enclosed squares")
    }


}

pub fn part1(path: &str) -> String {
    let lines: Vec<String> = utilities::string_iterator(path).collect();

    let map = Map::read(lines);
    let farthest_point = map.count_path();

    format!("The most distance point is {farthest_point} steps")
}

pub fn part2(path: &str) -> String {
    let lines: Vec<String> = utilities::string_iterator(path).collect();

    let map = Map::read(lines);
    // let count = map.count_enclosed();
    let count = map.count_enclosed_scan();

    format!("There are {count} enclosed squares")
}

struct Map {
    rows: usize,
    cols: usize,
    squares: Vec<MapSquare>
}

impl Map {
    fn read(lines: Vec<String>) -> Self {
        let mut cols: usize = 0;
        let mut rows: usize = 0;
        let mut squares: Vec<MapSquare> = Vec::new();

        for line in lines {
            rows += 1;
            let mut rowlen: usize = 0;
            for ch in line.chars() {
                rowlen += 1;
                squares.push(MapSquare::read_char(&ch));
            }
            
            if cols == 0 {
                cols = rowlen;
            } else if cols != rowlen {
                panic!("Read inconsistent row lengths")
            }

        }

        Self{ rows, cols,  squares}
    }

    fn find_path(&self) -> Vec<((usize, usize), MapSquare)> {
        let (start, mut position) = self.find_start();
        let mut prev = start;
        let mut path = vec![(start, MapSquare::Start)];
        while position != start {
            // println!("{start:?} {position:?} {steps}");
            path.push((position, self.get_square(position)));
            let adjacent = self.get_square(position).connected(position);
            let next_position = if adjacent.0 == prev {
                adjacent.1
            } else {
                adjacent.0
            };
            prev = position;
            position = next_position;
        }
        path[0] = (start, MapSquare::connecting(start, path[1].0, path.last().unwrap().0));

        path
    }

    fn count_path(&self) -> u32 {
        let path = self.find_path();

        path.len() as u32 / 2
    }

    fn count_enclosed_scan(&self) -> u32 {
        let mut map = self.squares.clone();
        let path = self.find_path();
        let (start_row, start_col) = path[0].0;
        let convert = |row: usize, col: usize| row*self.cols + col;
        map[convert(start_row, start_col)] = path[0].1;

        let path_idx: Vec<(usize, usize)> = path.iter().map(|tpl| tpl.0).collect();
        for i in 0..map.len() {
            let row = i / self.cols;
            let col = i % self.cols;
            if !path_idx.contains(&(row, col)) {
                map[i] = MapSquare::Ground;
            }
        }

        // println!("{}", map_string(&map, self.rows, self.cols));

        // println!("Sanity check: size is ({} {})", self.rows, self.cols);

        let mut outside = vec![true; self.cols];
        let mut last_turn: Vec<Option<MapSquare>> = vec![None; self.cols];
        let mut count: u32 = 0;

        for i in 0..self.rows {
            for j in 0..self.cols {
                let square = map[convert(i, j)];
                match square {
                    MapSquare::Horizontal => {
                        outside[j] = !outside[j];
                    },
                    MapSquare::Ground => {
                        if !outside[j] {
                            count += 1;
                        }
                    },
                    MapSquare::NE | MapSquare::SE => {
                        match last_turn[j] {
                            None => {
                                last_turn[j] = Some(square);
                            },
                            Some(MapSquare::NE) | Some(MapSquare::SE) => {
                                last_turn[j] = None;
                            },
                            Some(MapSquare::NW) | Some(MapSquare::SW) => {
                                last_turn[j] = None;
                                outside[j] = !outside[j];
                            },
                            _ => ()
                        }
                    },
                    MapSquare::NW | MapSquare::SW => {
                        match last_turn[j] {
                            None => {
                                last_turn[j] = Some(square);
                            },
                            Some(MapSquare::NE) | Some(MapSquare::SE) => {
                                outside[j] = !outside[j];
                                last_turn[j] = None;
                            },
                            Some(MapSquare::NW) | Some(MapSquare::SW) => {
                                last_turn[j] = None;
                            },
                            _ => ()
                        }
                    },
                    _ => ()
                }
            }
        }

        // println!("{outside:?}");
        // println!("{last_turn:?}");

        count
    }

    // fn count_enclosed(&self) -> u32 {
    //     let path = self.find_path();
    //     let mut enclosed: HashSet<(usize, usize)> = HashSet::new();

    //     // for each square on the path
    //     // move perpendicularly to the edge, counting times cross the path
    //     // the direction of the enclosure is the one with an odd number of crossings
    //     // add (.insert()) enclosed positions to list if new
    //     for (position, square) in &path {
    //         let mover = square.orth_mover();
    //         let mut row = position.0 as isize;
    //         let mut col = position.1 as isize;
    //         println!("position: {position:?}, square: {square:?}");
    //         let mut cross_squares: Vec<MapSquare> = Vec::new();
    //         loop {
    //             (row, col) = mover((row, col));
    //             if col >= 0 && col < self.cols as isize && row >= 0 && row < self.rows as isize {
    //                 if path.iter().any(|t| t.0 == (row as usize, col as usize)) {
    //                     let cross_square = self.get_square((row as usize, col as usize));
    //                     if cross_square != MapSquare::Start {
    //                         cross_squares.push(cross_square);
    //                     } else {
    //                         cross_squares.push(path[0].1);
    //                     }
    //                 }
    //             } else {
    //                 break;
    //             }
    //         }

    //         let cross_count = square.cross_count(&cross_squares);
    //         println!("{cross_count} crosses: {cross_squares:?}");

    //         if cross_count % 2 == 0 {
    //             let rev_mover = square.rev_orth_mover();
    //             let mut row = position.0 as isize;
    //             let mut col = position.1 as isize;
    //             loop {
    //                 (row, col) = rev_mover((row, col));
    //                 if path.iter().any(|t| t.0 == (row as usize, col as usize)) {
    //                     break;
    //                 } else {
    //                     enclosed.insert((row as usize, col as usize));
    //                 }
    //             }
    //         } else {
    //             let mut row = position.0 as isize;
    //             let mut col = position.1 as isize;

    //             loop {
    //                 (row, col) = mover((row, col));
    //                 if path.iter().any(|t| t.0 == (row as usize, col as usize)) {
    //                     break;
    //                 } else {
    //                     enclosed.insert((row as usize, col as usize));
    //                 }
    //             }
    //         }
    //     }

    //     println!("{enclosed:?}");

    //     enclosed.len() as u32
    // }

    fn get_square(&self, idx: (usize, usize)) -> MapSquare {
        let (row, col) = idx;
        self.squares[row*self.cols + col]
    }

    fn north(&self, idx: (usize, usize)) -> Option<((usize, usize), MapSquare)> {
        let (row, col) = idx;
        if row == 0 {
            None
        } else {
            let position = (row-1, col);
            Some((position, self.get_square(position)))
        }
    }

    fn south(&self, idx: (usize, usize)) -> Option<((usize, usize), MapSquare)> {
        let (row, col) = idx;
        if row == self.rows-1 {
            None
        } else {
            let position = (row+1, col);
            Some((position, self.get_square(position)))
        }
    }

    fn east(&self, idx: (usize, usize)) -> Option<((usize, usize), MapSquare)> {
        let (row, col) = idx;
        if col == self.cols - 1 {
            None
        } else {
            let position = (row, col+1);
            Some((position, self.get_square(position)))
        }
    }

    fn west(&self, idx: (usize, usize)) -> Option<((usize, usize), MapSquare)> {
        let (row, col) = idx;
        if col == 0 {
            None
        } else {
            let position = (row, col-1);
            Some((position, self.get_square(position)))
        }
    }

    /// Returns the location of the start point and one of the pipes connected to it
    fn find_start(&self) -> ((usize, usize), (usize, usize)) {
        let mut i: usize = 0;
        while self.squares[i] != MapSquare::Start {
            i += 1;
        }
        let start = (i / self.cols, i % self.cols);

        if let Some((north_idx, north_square)) = self.north(start) {
            if north_square.south_connection() {
                return (start, north_idx)
            }
        }

        if let Some((south_idx, south_square)) = self.south(start) {
            if south_square.north_connection() {
                return (start, south_idx)
            }
        }

        if let Some((east_idx, east_square)) = self.east(start) {
            if east_square.west_connection() {
                return (start, east_idx)
            }
        }

        if let Some((west_idx, west_square)) = self.west(start) {
            if west_square.east_connection() {
                return (start, west_idx)
            }
        }

        panic!("Couldn't find a connecting square for start")
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum MapSquare {
    Vertical,
    Horizontal,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start    
}

impl MapSquare {
    fn read_char(ch: &char) -> Self {
        match ch {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::NE,
            'J' => Self::NW,
            '7' => Self::SW,
            'F' => Self::SE,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => panic!("Invalid character")
        }
    }

    fn connecting(node: (usize, usize), arm1: (usize, usize), arm2: (usize, usize)) -> Self {
        let node_to_arm1 = offset(node, arm1);
        let node_to_arm2 = offset(node, arm2);

        match (node_to_arm1, node_to_arm2) {
            ((0,1), (0,-1))|((0, -1), (0, 1)) => Self::Horizontal,
            ((1,0), (-1,0))|((-1, 0), (1 ,0)) => Self::Vertical,
            ((0, 1), (-1,0))|((-1, 0), (0, 1)) => Self::NE,
            ((0, -1), (-1,0))|((-1, 0), (0, -1)) => Self::NW,
            ((0, 1), (1,0))|((1, 0), (0, 1)) => Self::SE,
            ((0, -1), (1,0))|((1, 0), (0, -1)) => Self::SW,
            _ => panic!("Can't connect indices")
        }
    }

    fn connected(&self, idx: (usize, usize)) -> ((usize, usize), (usize, usize)) {
        let (row, col) = idx;
        match self {
            Self::Vertical => ((row+1, col), (row-1, col)),
            Self::Horizontal => ((row, col+1), (row, col-1)),
            Self::NE => ((row-1, col), (row, col+1)),
            Self::NW => ((row-1, col), (row, col-1)),
            Self::SE => ((row+1, col), (row, col+1)),
            Self::SW => ((row+1, col), (row, col-1)),
            _ => panic!("Trying to find adjacent for ground square or start point")
        }
    }

    fn south_turn(&self) -> bool {
        *self == Self::SE || *self == Self::SW
    }

    fn north_turn(&self) -> bool {
        *self == Self::NE || *self == Self::NW
    }

    fn east_turn(&self) -> bool {
        *self == Self::NE || *self == Self::SE
    }

    fn west_turn(&self) -> bool {
        *self == Self::NW || *self == Self::SW
    }

    fn south_connection(&self) -> bool {
        *self == Self::Vertical || self.south_turn()
    }


    fn north_connection(&self) -> bool {
        *self == Self::Vertical || self.north_turn()
    }

    fn east_connection(&self) -> bool {
        *self == Self::Horizontal || self.east_turn()
    }

    fn west_connection(&self) -> bool {
        *self == Self::Horizontal || self.west_turn()
    }

//     fn orth_mover(&self) -> impl Fn((isize, isize)) -> (isize, isize) {
//         match self {
//             Self::Horizontal => |(row, col): (isize, isize)| {(row-1, col)},
//             Self::Vertical => |(row, col): (isize, isize)| {(row, col-1)},
//             Self::NE | Self::SW => |(row, col): (isize, isize)| {(row-1, col+1)},
//             Self::NW | Self::SE => |(row, col): (isize, isize)| {(row-1, col-1)},
//             _ => panic!("Can't move orthogonally from non-pipe square")
//         }
//     }

//     fn rev_orth_mover(&self) -> impl Fn((isize, isize)) -> (isize, isize) {
//         match self {
//             Self::Horizontal => |(row, col): (isize, isize)| {(row+1, col)},
//             Self::Vertical => |(row, col): (isize, isize)| {(row, col+1)},
//             Self::NE | Self::SW => |(row, col): (isize, isize)| {(row+1, col-1)},
//             Self::NW | Self::SE => |(row, col): (isize, isize)| {(row+1, col+1)},
//             _ => panic!("Can't move orthogonally from non-pipe square")
//         }
//     }

//     fn cross_count(&self, cross_squares: &Vec<Self>) -> u32 {
//         match self {
//             Self::Horizontal => {
//                 let simple_crosses = cross_squares.iter().filter(|s| **s == Self::Horizontal).count();
//                 let east_turns = cross_squares.iter().filter(|s| s.east_turn()).count();
//                 let west_turns = cross_squares.iter().filter(|s| s.west_turn()).count();
//                 (simple_crosses + east_turns.min(west_turns)) as u32   
//             },
//             Self::Vertical => {
//                 let simple_crosses = cross_squares.iter().filter(|s| **s == Self::Vertical).count();
//                 let north_turns = cross_squares.iter().filter(|s| s.north_turn()).count();
//                 let south_turns = cross_squares.iter().filter(|s| s.south_turn()).count();
//                 (simple_crosses + north_turns.min(south_turns)) as u32             
//             },
//             Self::NE | Self::SW => {
//                 let counter = |sq: &&MapSquare| {
//                     **sq == Self::Vertical ||
//                     **sq == Self::Horizontal ||
//                     **sq == Self::NE ||
//                     **sq == Self::SW
//                 };

//                 cross_squares.iter().filter(counter).count() as u32
//             },
//             Self::NW | Self::SE => {
//                 let counter = |sq: &&MapSquare| {
//                     **sq == Self::Vertical ||
//                     **sq == Self::Horizontal ||
//                     **sq == Self::NW ||
//                     **sq == Self::SE
//                 };

//                 cross_squares.iter().filter(counter).count() as u32
//             },
//             _ => panic!("Can't cross starting from non-pipe square")
//         }
//     }
}

/// Returns difference `another - one`
fn offset(one: (usize, usize), another: (usize, usize)) -> (isize, isize) {
    (another.0 as isize - one.0 as isize, another.1 as isize - one.1 as isize)
}

// fn map_string(map: &Vec<MapSquare>, rows: usize, cols: usize) -> String {
//     let mut mapstr = String::new();
//     for i in 0..rows {
//         for j in 0..cols {
//             let idx = i*cols + j;
//             let ch = match map[idx] {
//                 MapSquare::Ground => '.',
//                 MapSquare::Horizontal => '-',
//                 MapSquare::NE => 'L',
//                 MapSquare::NW => 'J',
//                 MapSquare::SE => 'F',
//                 MapSquare::SW => '7',
//                 MapSquare::Start => 'S',
//                 MapSquare::Vertical => '|',
//             };
//             mapstr.push(ch);
//         }
//         mapstr.push('\n');
//     }

//     mapstr
// }