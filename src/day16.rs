use crate::utilities;

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn part1_test1() {
        let result = part1("./input/day16_test1.txt");

        assert_eq!(result, "The number of energized tiles is 46");
    }

    #[test]
    fn part2_test1() {
        let result = part2("./input/day16_test1.txt");

        assert_eq!(result, "The max number of energized tiles is 51");
    }
}

pub fn part1(path: &str) -> String {
    let lines = utilities::string_iterator(path);
    let mut grid = Grid::read(lines);
    grid.start_following_beams();
    let count = grid.count_energized();

    format!("The number of energized tiles is {count}")
}

pub fn part2(path: &str) -> String {
    let lines = utilities::string_iterator(path);
    let grid = Grid::read(lines);
    let count = grid.find_most_energized();

    format!("The max number of energized tiles is {count}")
}

#[derive(Clone)]
struct Grid {
    grid: Vec<Tile>,
    numrows: usize,
    numcols: usize
}

impl Grid {
    fn read(lines: impl Iterator<Item = String>) -> Self {
        let mut grid: Vec<Tile> = Vec::new();
        let mut numrows: usize = 0;
        let mut numcols: usize = 0;
        for line in lines {
            numrows += 1;
            let mut count: usize = 0;
            for ch in line.chars() {
                grid.push(Tile::read(ch));
                count += 1;
            }
            numcols = count;
        }
    
        Self {grid, numcols, numrows}
    }

    fn start_following_beams(& mut self) {
        self.follow_beam(0, Direction::Right)
    }

    fn find_most_energized(& self) -> u32 {
        let mut max_energized: u32 = 0;
        for i in 0..self.numcols {
            let mut fresh_grid = self.clone();
            fresh_grid.follow_beam(i, Direction::Down);
            max_energized = max_energized.max(fresh_grid.count_energized());

            let mut fresh_grid = self.clone();
            fresh_grid.follow_beam(self.grid.len()-1-i, Direction::Up);
            max_energized = max_energized.max(fresh_grid.count_energized());
        }

        for i in 0..self.numrows {
            let mut fresh_grid = self.clone();
            fresh_grid.follow_beam(i*self.numcols, Direction::Right);
            max_energized = max_energized.max(fresh_grid.count_energized());

            let mut fresh_grid = self.clone();
            fresh_grid.follow_beam((i+1)*self.numcols - 1, Direction::Left);
            max_energized = max_energized.max(fresh_grid.count_energized());
        }

        max_energized
    }

    fn follow_beam(& mut self, index:usize, dir: Direction) {
        if self.grid[index].has_beam(dir) {
            return;
        } else {
            // println!("Adding {dir:?} to {index}");
            self.grid[index].add_beam(dir);
        }
        
        match self.grid[index].ch {
            '.' => {
                if let Some(new_index) = self.move_1(index, dir) {
                    self.follow_beam(new_index, dir)
                }
            },
            '\\' => {
                let new_dir = match dir {
                    Direction::Down => {
                        Direction::Right
                    },
                    Direction::Left => {
                        Direction::Up
                    },
                    Direction::Right => {
                        Direction::Down
                    },
                    Direction::Up => {
                        Direction::Left
                    },
                };
                if let Some(new_index) = self.move_1(index, new_dir) {
                    self.follow_beam(new_index, new_dir)
                }
            },
            '/' => {
                let new_dir = match dir {
                    Direction::Down => {
                        Direction::Left
                    },
                    Direction::Left => {
                        Direction::Down
                    },
                    Direction::Right => {
                        Direction::Up
                    },
                    Direction::Up => {
                        Direction::Right
                    },
                };
                if let Some(new_index) = self.move_1(index, new_dir) {
                    self.follow_beam(new_index, new_dir)
                }
            },
            '-' => {
                match dir {
                    Direction::Down | Direction::Up => {
                        let new_dir = Direction::Left;
                        if let Some(new_index) = self.move_1(index, new_dir) {
                            self.follow_beam(new_index, new_dir)
                        }
                        let new_dir = Direction::Right;
                        if let Some(new_index) = self.move_1(index, new_dir) {
                            self.follow_beam(new_index, new_dir)
                        }
                    },
                    Direction::Left | Direction::Right => {
                        if let Some(new_index) = self.move_1(index, dir) {
                            self.follow_beam(new_index, dir)
                        }
                    }
                }
            },
            '|' => {
                match dir {
                    Direction::Left | Direction::Right => {
                        let new_dir = Direction::Up;
                        if let Some(new_index) = self.move_1(index, new_dir) {
                            self.follow_beam(new_index, new_dir)
                        }
                        let new_dir = Direction::Down;
                        if let Some(new_index) = self.move_1(index, new_dir) {
                            self.follow_beam(new_index, new_dir)
                        }
                    },
                    Direction::Down | Direction::Up => {
                        if let Some(new_index) = self.move_1(index, dir) {
                            self.follow_beam(new_index, dir)
                        }
                    }
                }
            },
            _ => {
                panic!("Invalid character");
            },
        }
    }

    fn move_1(&self, index:usize, dir:Direction) -> Option<usize> {
        match dir {
            Direction::Down => {
                let new_pos = index + self.numcols;
                if new_pos < self.grid.len() {
                    Some(new_pos)
                } else {
                    None
                }
            },
            Direction::Left => {
                if index % self.numcols == 0 {
                    None
                } else {
                    Some(index - 1)
                }
            },
            Direction::Right => {
                if (index + 1) % self.numcols == 0 {
                    None
                } else {
                    Some(index + 1)
                }
            },
            Direction::Up => {
                if index < self.numcols {
                    None
                } else {
                    Some(index - self.numcols)
                }
            },
        }
    }

    fn count_energized(& self) -> u32 {
        self.grid
            .iter()
            .filter(
                | t | {
                    t.energized()
                }
            )
            .count() as u32
    }

}

#[derive(Clone)]
struct Tile {
    ch: char,
    beams: [bool; 4]
}

impl Tile {
    fn read(ch: char) -> Self {
        Self { ch, beams:[false; 4] }
    }

    fn add_beam(& mut self, dir: Direction) {
        self.beams[dir.as_index()] = true;
    }

    fn has_beam(& self, dir: Direction) -> bool {
        self.beams[dir.as_index()]
    }

    fn energized(& self) -> bool {
        self.beams.iter().any(| dir | *dir)
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn as_index(&self) -> usize {
        match self {
            Self::Down => 0,
            Self::Up => 1,
            Self::Left => 2,
            Self::Right => 3
        }
    }
}