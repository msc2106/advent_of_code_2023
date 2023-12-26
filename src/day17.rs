use core::panic;

use crate::utilities;

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn part1_test1() {
        let result = part1("./input/day17_test1.txt");

        assert_eq!(result, "The minimum heat loss is 102")
    }

    #[test]
    fn part2_test1() {
        let result = part2("./input/day17_test1.txt");

        assert_eq!(result, "The minimum heat loss is 94")
    }

    #[test]
    fn part2_test2() {
        let result = part2("./input/day17_test2.txt");

        assert_eq!(result, "The minimum heat loss is 71")
    }
}

pub fn part1(path: &str) -> String {
    let lines = utilities::string_iterator(path);
    let mut finder = PathFinder::read(lines);
    let min_loss = finder.find_path();

    format!("The minimum heat loss is {min_loss}")
}

pub fn part2(path: &str) -> String {
    let lines = utilities::string_iterator(path);
    let mut finder = PathFinder::read(lines);
    let min_loss = finder.find_path_ultra();
    // for rownum in 0..finder.numrows {
    //     let mut line_str = String::new();
    //     for colnum in 0..finder.numcols {
    //         let index = rownum * finder.numcols + colnum;
    //         let cost = finder.get_cost_from_direction(index, Direction::Left);
    //         let entry_str = format!("{cost} ");
    //         line_str.push_str(&entry_str);
    //     }
    //     println!("{line_str}");
    // }
    // println!("");
    // for rownum in 0..finder.numrows {
    //     let mut line_str = String::new();
    //     for colnum in 0..finder.numcols {
    //         let index = rownum * finder.numcols + colnum;
    //         let cost = finder.get_cost_from_direction(index, Direction::Up);
    //         let entry_str = format!("{cost} ");
    //         line_str.push_str(&entry_str);
    //     }
    //     println!("{line_str}");
    // }

    format!("The minimum heat loss is {min_loss}")
}

struct PathFinder {
    city: Vec<u32>,
    losses: Vec<Vec<PathInfo>>,
    numrows: usize,
    numcols: usize
}

impl PathFinder {
    fn read(lines: impl Iterator <Item = String>) -> Self {
        let mut city: Vec<u32> = Vec::new();
        let mut numrows = 0;

        for line in lines {
            numrows += 1;
            let mut new_blocks: Vec<u32> = line.chars().filter_map(| ch | ch.to_digit(10)).collect();
            city.append(&mut new_blocks);
        }

        let numcols = city.len() / numrows;
        let losses: Vec<Vec<PathInfo>> = vec![Vec::new(); city.len()];

        Self { city, losses, numrows, numcols }
    }

    fn find_path(& mut self) -> u32 {
        let starting_point = PathInfo {
            cost: 0,
            direction: Direction::Left,
            reps: 0
        };
        self.losses[0] = vec![starting_point.clone()];

        let (_, mut adj_stack) = self.adjacent(0);

        while adj_stack.len() > 0 {
            // pop from stack
            let this_idx = adj_stack.pop().unwrap();
            // println!("This idx: {}, Stack remaining:{:?}", this_idx, adj_stack.len());
            let added_cost = self.city[this_idx];
            let mut new_info: Vec<PathInfo> = Vec::new();
            // look at each neighbor
            let (neighbors_dir, mut neighbors_idx) = self.adjacent(this_idx);
            for (neighbor_direction, neighbor_idx) in neighbors_dir.iter().zip(neighbors_idx.iter()) {
                let neighbor_paths = &self.losses[*neighbor_idx];
                // save shortest path from each direction
                let shortest_path = neighbor_paths
                    .iter()
                    .filter(| info | {
                        info.direction != neighbor_direction.opposite() &&
                        (info.direction != *neighbor_direction || info.reps < 3)
                    })
                    .min_by_key(| info | info.cost);
                if let Some(path) = shortest_path {
                    let link = path.link(*neighbor_direction, added_cost);
                    // if this is two steps in the same direction, also save the shortest that is not
                    if link.reps > 1 {
                        let shortest_nonstraight_path = neighbor_paths
                            .iter()
                            .filter(| info | {
                                info.direction != neighbor_direction.opposite() &&
                                (info.direction != *neighbor_direction || info.reps < 1)
                            })
                            .min_by_key(| info | info.cost);
                        if let Some(nonstraight_path) = shortest_nonstraight_path {
                            new_info.push(nonstraight_path.link(*neighbor_direction, added_cost));
                        }
                    }

                    if link.reps == 3 {
                        let shortest_nonstraight_path = neighbor_paths
                            .iter()
                            .filter(| info | {
                                info.direction != neighbor_direction.opposite() &&
                                (info.direction != *neighbor_direction || info.reps < 2)
                            })
                            .min_by_key(| info | info.cost);
                        if let Some(nonstraight_path) = shortest_nonstraight_path {
                            new_info.push(nonstraight_path.link(*neighbor_direction, added_cost));
                        }
                    }
                    new_info.push(link);
                }
            }
            // need to keep starting point
            if this_idx == 0 {
                new_info.push(starting_point.clone());
            }
            // sort paths and compare to saved info
            // if any change, replace and add all neighbors to stack
            new_info.sort();
            if self.losses[this_idx] != new_info {
                // println!("Updating {}", this_idx);
                // println!("Old: {:?}", self.losses[this_idx]);
                // println!("New: {:?}", new_info);
                if cost_increasing(&self.losses[this_idx], &new_info) {
                    panic!("Found increasing cost");
                }
                self.losses[this_idx] = new_info;
                adj_stack.append(&mut neighbors_idx);
                adj_stack.sort();
                adj_stack.reverse();
            }
        }
        // println!("{:?}", self.losses.last());
        self.losses
            .last()
            .unwrap()
            .iter()
            .map(| info | info.cost)
            .min()
            .unwrap()
    }


    fn move_n_to_m(& self, index: usize, direction: Direction, start: usize, end: usize) -> Vec<usize> {
        let start_row = index / self.numcols;
        let start_col = index % self.numcols;
      
        match direction {
            Direction::Down => {
                let valid = | i: &usize | {
                    start_row + *i < self.numrows
                };
                let stepper = | i: usize | {
                    index + i * self.numcols
                };
                (0..self.numrows)
                    .filter(valid)
                    .map(stepper)
                    .skip(start)
                    .take(end - start + 1)
                    .collect()
            },
            Direction::Left => {
                let valid = | i: &usize | {
                    *i <= start_col
                };
                let stepper = | i: usize | {
                    index - i
                };
                (0..self.numcols)
                    .filter(valid)
                    .map(stepper)
                    .skip(start)
                    .take(end - start + 1)
                    .collect()
            },
            Direction::Right => {
                let valid = | i: &usize | {
                    start_col + *i < self.numcols
                };
                let stepper = | i: usize | {
                    index + i
                };
                (0..self.numcols)
                    .filter(valid)
                    .map(stepper)
                    .skip(start)
                    .take(end - start + 1)
                    .collect()
            },
            Direction::Up => {
                let valid = | i: &usize | {
                    *i <= start_row
                };
                let stepper = | i: usize | {
                    index - i * self.numcols
                };
                (0..self.numrows)
                    .filter(valid)
                    .map(stepper)
                    .skip(start)
                    .take(end - start + 1)
                    .collect()
            },
        }
    }


    // fn move_right(& self, index: usize) -> Vec<usize> {
    //     let mut turn_points: Vec<usize> = Vec::new();

    //     if index % self.numcols + 4 < self.numcols {
    //         let first_point = index + 4;
    //         let last_point = (index + 10).min((index/self.numcols+1)*self.numcols-1);
    //         for turn_point in first_point..=last_point {
    //             turn_points.push(turn_point);
    //         }
    //     }

    //     turn_points
    // }

    // fn move_down(& self, index:usize) -> Vec<usize> {
    //     let turn_points: Vec<usize> = (index+4*self.numcols..self.city.len())
    //         .filter(| i | {
    //             *i % self.numcols == index % self.numcols
    //         })
    //         .take(7)
    //         .collect();
        
    //     turn_points
    // }

    fn get_cost_to_direction(& self, index: usize, direction: Direction) -> u32 {
        self
            .losses[index]
            .iter()
            .filter(| info | info.direction.is_orth(direction))
            .map(| info | info.cost)
            .min()
            .unwrap_or(self.city[index])
    }

    fn update_cost_from_direction(& mut self, index: usize, direction: Direction, cost: u32) -> bool {
        let mut found_direction = false;
        let mut updated = false;
        for current_info in & mut self.losses[index] {
            if current_info.direction == direction {
                found_direction = true;
                if current_info.cost > cost {
                    current_info.cost = cost;
                    updated = true;
                }
            }
        }
        if !found_direction {
            self.losses[index].push(PathInfo {
                direction,
                cost,
                reps: 1, // does not matter here
            });
            updated = true;
        }
        updated
    }

    fn find_path_ultra(& mut self) -> u32 {
        let starting_points = vec![
            PathInfo {
                cost: 0,
                direction: Direction::Up,
                reps: 0 
            },
            PathInfo {
                cost: 0,
                direction: Direction::Left,
                reps: 0 
            }
        ];
        self.losses[0] = starting_points.clone();
        let mut current_best: Option<u32> = None;

        // cheating with known high estimate
        // let mut current_best: Option<u32> = Some(1000);
        // let n = self.losses.len();
        // self.losses[n - 1] = vec![PathInfo {
        //     cost: 1000,
        //     direction: Direction::Down,
        //     reps: 1
        // }];
        // ---

        let mut turns: Vec<(usize, Direction)> = vec![
            (0, Direction::Right),
            (0, Direction::Down)
        ];
        // for each turn point:
        //  1. identify possible next turns (4 to 10 moves)
        //  2. calculate cost, and replace in losses grid if less
        //  3. add turns to appropriate stack
        while turns.len() > 0 {
            let (turn, direction) = turns.pop().unwrap();

            let mut cumulative_cost: u32 = self.get_cost_to_direction(turn, direction);
            cumulative_cost += self.move_n_to_m(turn, direction, 1, 3)
                .into_iter()
                .map(| index | self.city[index])
                .sum::<u32>();

            let next_turns = self.move_n_to_m(turn, direction, 4, 10);
            for turning_point in &next_turns {
                if let Some(best) = current_best {
                    if best <= cumulative_cost {
                        break;
                    }
                }
                cumulative_cost += self.city[*turning_point];
                if self.update_cost_from_direction(
                    *turning_point, 
                    direction.opposite(),
                    cumulative_cost
                ) {
                    match direction {
                        Direction::Down | Direction::Up => {
                            turns.push((*turning_point, Direction::Left));
                            turns.push((*turning_point, Direction::Right))
                        },
                        Direction::Left | Direction::Right => {
                            turns.push((*turning_point, Direction::Up));
                            turns.push((*turning_point, Direction::Down))
                        }
                    }
                }
            }
            current_best = self.losses
                .last()
                .unwrap()
                .iter()
                .map(| info | info.cost)
                .min();
            // println!("{:?}, {}", current_best, turns.len());
            if current_best.is_none() {
                turns.sort_by_key(| (index, _) | *index);
            }
        }

        self.losses
            .last()
            .unwrap()
            .iter()
            .map(| info | info.cost)
            .min()
            .unwrap()
    }

    fn adjacent(&self, index: usize) -> (Vec<Direction>, Vec<usize>) {
        let mut adj_stack = Vec::new();
        let mut directions: Vec<Direction> = Vec::new();
        if index >= self.numcols {
            adj_stack.push(index - self.numcols);
            directions.push(Direction::Up);
        }

        if index % self.numcols != 0 {
            adj_stack.push(index - 1);
            directions.push(Direction::Left);
        }

        if (index + 1) % self.numcols != 0 {
            adj_stack.push(index + 1);
            directions.push(Direction::Right);
        }

        if index / self.numcols < self.numrows - 1 {
            adj_stack.push(index + self.numcols);
            directions.push(Direction::Down);
        }

        (directions, adj_stack)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct PathInfo {
    direction: Direction,
    reps: usize,
    cost: u32
}

impl PathInfo {
    fn link(&self, direction: Direction, added_cost: u32) -> Self {
        let new_cost = self.cost + added_cost;
        let new_reps = if self.direction == direction {
            self.reps + 1
        } else {
            1
        };
        Self {
            cost: new_cost,
            direction,
            reps: new_reps
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down
        }
    }

    fn is_orth(& self, other: Direction) -> bool {
        if *self == other || self.opposite() == other {
            false
        } else {
            true
        }
    }
}

fn cost_increasing (old_losses: &Vec<PathInfo>, new_losses: &Vec<PathInfo>) -> bool {
    for (old, new) in old_losses.iter().zip(new_losses.iter()) {
        if new.reps == old.reps && new.direction == old.direction && new.cost > old.cost {
            return true;
        }
    }

    false
}
