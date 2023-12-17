use std::{fmt::Debug, collections::HashMap};
// use std::iter;

use crate::utilities;

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn part1_test_1() {
        let result = part1("./input/day12_test1.txt");

        assert_eq!(result, "The total number of possibilities is 21")
    }

    #[test]
    fn part2_test_1() {
        let result = part2("./input/day12_test1.txt");

        assert_eq!(result, "The total number of possibilities is 525152")
    }
}

pub fn part1(path: &str) -> String {
    let lines = utilities::string_iterator(path);
    
    let fixers = lines.map(|line| Fixer::read(line));

    let total: u64 = fixers.map(|f| f.find_fixes_naive()).sum();

    format!("The total number of possibilities is {total}")
}

pub fn part2(path: &str) -> String {
    let lines = utilities::string_iterator(path);
    
    // let fixers = lines.map(|line| Fixer::read(line));
    let fixers = lines.map(|line| Fixer::read_expanded(line));

    let total: u64 = fixers.map(|f| f.find_fixes()).sum();

    format!("The total number of possibilities is {total}")
}

struct Fixer {
    entries: Vec<Item>,
    blocks: Vec<usize>,
}

impl Fixer {
    fn read(line: String) -> Self {
        let mut parts = line.split(' ');
        let entries: Vec<Item> = parts.next().unwrap()
                                        .chars()
                                        .map(| ch | Item::from(ch))
                                        .collect();
        let blocks: Vec<usize> = parts.next().unwrap()
                                    .split(',')
                                    .filter_map(| s | s.parse().ok())
                                    .collect();
        Self {entries, blocks }
    }

    fn read_expanded(line: String) -> Self {
        let mut parts = line.split(' ');
        let entries_base: Vec<Item> = parts.next().unwrap()
                                        .chars()
                                        .map(| ch | Item::from(ch))
                                        .collect();
        let blocks_base: Vec<usize> = parts.next().unwrap()
                                    .split(',')
                                    .filter_map(| s | s.parse().ok())
                                    .collect();
        
        let mut entries = entries_base.clone();
        let mut blocks = blocks_base.clone();
        for _ in 0..4 {
            entries.push(Item::Unkown);
            entries.append(&mut entries_base.clone());
            blocks.append(&mut blocks_base.clone());
        }
        // println!("{:?} {:?}", entries, blocks);
        Self {entries, blocks }
    }

    fn find_fixes_naive(&self) -> u64 {
        let unknowns: Vec<usize> = self.entries.iter().enumerate().filter(|(_, e)| e.is_unknown()).map(|(i, _)| i).collect();
        let count_known_broken = self.entries.iter().filter(| e | e.is_broken()).count();
        let n_choices = unknowns.len();
        let choose_k = self.blocks.iter().sum::<usize>() - count_known_broken;
        println!("{n_choices} choose {choose_k}");

        let starter: Vec<usize> = Vec::with_capacity(choose_k);
        let valid = self.check_all_choices(unknowns.clone(), starter, choose_k);
        
        valid
    }

    fn check_all_choices(&self, mut unknowns: Vec<usize>, mut starter: Vec<usize>, choose_k: usize) -> u64 {
        if starter.len() == choose_k {
            let seq = self.fill_broken(&starter);
            if self.valid_sequence(&seq) {
                // println!("{seq:?} VALID");
                return 1;
            } else {
                // println!("{seq:?} INVALID");
                return 0;
            }
        } else if unknowns.len() == choose_k - starter.len() {
            starter.append(& mut unknowns);
            let seq = self.fill_broken(&starter);
            if self.valid_sequence(&seq) {
                // println!("{seq:?} VALID");
                return 1;
            } else {
                // println!("{seq:?} INVALID");
                return 0;
            }
        }

        let mut valid: u64 = 0;

        while unknowns.len() >= choose_k - starter.len() {
            let mut pass_on = starter.clone();
            pass_on.push(unknowns.pop().expect("Ran out of unknowns"));
            valid += self.check_all_choices(unknowns.clone(), pass_on, choose_k);
        }

        valid
    }

    fn fill_broken(&self, indices: &Vec<usize>) -> Vec<Item> {
        let replacer = | (idx, e): (usize, Item) | {
            if indices.contains(&idx) {
                Item::Broken
            } else if e.is_unknown() {
                Item::Working
            } else {
                e
            }
        };

        let seq: Vec<Item> = self.entries.clone()
            .into_iter()
            .enumerate()
            .map(replacer)
            .collect();
        
        seq
    }

    // fn find_fixes(&self) -> u32 {
    //     let unknowns = self.entries.iter().enumerate().filter(|(_, e)| e.is_unknown()).map(|(i, _)| i).count();
    //     let n_possibilities = 2u64.pow(unknowns as u32);
    //     let mut possibility_iterators: Vec<Box<dyn Iterator<Item = Item>>> = Vec::new();
    //     println!("Considering {:?} with {}", self.entries, n_possibilities);
        
    //     let mut freq:usize = 1;
    //     for element in &self.entries {
    //         if element.is_unknown() {
    //             let working_part = iter::repeat(Item::Working).take(freq);
    //             let broken_part = iter::repeat(Item::Broken).take(freq);
    //             let repeater = working_part.chain(broken_part).cycle();
    //             possibility_iterators.push(Box::new(repeater));
    //             freq *= 2;
    //         } else {
    //             possibility_iterators.push(Box::new(iter::repeat(*element)));
    //         }
    //     }
    //     let mut valid: u32 = 0;
    //     for _ in 0..n_possibilities {
    //         // println!("{i}");
    //         let seq = possibility_iterators.iter_mut().map(|it| it.next().unwrap()).collect();
    //         if self.valid_sequence(&seq) {
    //             valid += 1;
    //         }
    //     }

    //     valid
    // }

    fn next_broken_index(&self, seq: &Vec<Item>, start: usize) -> usize {
        seq[start..seq.len()]
            .iter()
            .position(|e| e.is_broken())
            .unwrap_or(seq.len())
    }

    fn valid_sequence(&self, seq: &Vec<Item>) -> bool {
        let n = seq.len();
        let mut blocks_iter = self.blocks.iter();
        let mut current_block = *blocks_iter.next().expect("Couldn't get first block");
        let mut i = self.next_broken_index(&seq, 0);

        while i < n {
            let item = seq[i];
            if item.is_working() {
                if current_block == 0 {
                    // move on to the next block
                    if let Some(next_block) = blocks_iter.next() {
                        current_block = *next_block;
                        i += self.next_broken_index(&seq, i);
                    } else {
                        // out of blocks:
                        // if there are no more broken, the sequence is valid
                        // otherwise it is not
                        return seq[i..n].iter().all(|e| e.is_working());
                    }
                } else {
                    // The block is too short
                    return false;
                }
            } else {
                if current_block == 0 {
                    // the block is too long
                    return false;
                } else {
                    // keep moving
                    current_block -= 1;
                    i += 1;
                }
            }
        }
        
        // if there are remaining blocks, the sequence is not valid
        current_block == 0 && blocks_iter.count() == 0

    }


    fn find_fixes(&self) -> u64 {
        let start_status = ScanPosition::new();
        let mut memo: HashMap<ScanPosition, u64> = HashMap::new();
        memo = self.recursive_scan(start_status, memo);
        // println!("{}", memo[&start_status]);
        memo[&start_status]
    }

    fn recursive_scan(&self, status: ScanPosition, mut memo:HashMap<ScanPosition, u64>) -> HashMap<ScanPosition, u64> {
        if memo.contains_key(&status) {
            (); // do nothing
        }
        else if status.seq_idx == self.entries.len() {
            if (status.block_idx == self.blocks.len() && status.broken_seen == 0)
                ||
                (status.block_idx == self.blocks.len()-1 && status.broken_seen == self.blocks[status.block_idx])
            {
                memo.insert(status, 1);
            } else {
                memo.insert(status, 0);
            }
        } else {
            // since unknown items are to be considered as either, these if statements are non-exclusive
            let current = self.entries[status.seq_idx];
            let mut valid_possibilities: u64 = 0;
            if current.is_broken() || current.is_unknown() {
                let new_status = status.saw_broken();
                if new_status.block_idx < self.blocks.len() 
                    && new_status.broken_seen <= self.blocks[new_status.block_idx] {
                    memo = self.recursive_scan(new_status, memo);
                    valid_possibilities += memo[&new_status];
                }
            }

            if current.is_working() || current.is_unknown() {
                if status.block_idx == self.blocks.len() || status.broken_seen == 0 {
                    let new_status = status.advance();
                    memo = self.recursive_scan(new_status, memo);
                    valid_possibilities += memo[&new_status];
                }
                else if status.broken_seen == self.blocks[status.block_idx] {
                    let new_status = status.next_block();
                    memo = self.recursive_scan(new_status, memo);
                    valid_possibilities += memo[&new_status];
                }
            }
            memo.insert(status, valid_possibilities);
        }
        // println!("{:?}: {}", status, memo[&status]);
        memo
    }

}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct ScanPosition {
    seq_idx: usize,
    block_idx: usize,
    broken_seen: usize,
}

impl  ScanPosition {
    fn new() -> Self {
        Self {
            seq_idx: 0,
            block_idx: 0,
            broken_seen: 0
        }
    }

    fn saw_broken(&self) -> Self {
        Self {
            seq_idx: self.seq_idx + 1,
            block_idx: self.block_idx,
            broken_seen: self.broken_seen + 1
        }
    }

    fn next_block(&self) -> Self {
        Self {
            seq_idx: self.seq_idx + 1,
            block_idx: self.block_idx + 1,
            broken_seen: 0
        }
    }

    fn advance(&self) -> Self {
        Self {
            seq_idx: self.seq_idx + 1,
            block_idx: self.block_idx,
            broken_seen: self.broken_seen
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Item {
    Broken,
    Working,
    Unkown
}

impl Item {
    fn is_broken(&self) -> bool {
        *self == Self::Broken
    }

    fn is_working(&self) -> bool {
        *self == Self::Working
    }

    fn is_unknown(&self) -> bool {
        *self == Self::Unkown
    }
}

impl From<char> for Item {
    fn from(value: char) -> Self {
        match value {
            '#' => Self::Broken,
            '.' => Self::Working,
            '?' => Self::Unkown,
            _ => panic!("Bad character")
        }
    }
}

impl Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = match self {
            Self::Broken => '#',
            Self::Unkown => '?',
            Self::Working => '.',
        };
        write!(f, "{}", ch)
    }
}
