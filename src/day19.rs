use crate::utilities;
use core::panic;
use std::collections::HashMap;

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn part1_test1() {
        let result = part1("./input/day19_test1.txt");

        assert_eq!(result, "The sum of ratings is 19114");
    }

    #[test]
    fn part2_test1() {
        let result = part2("./input/day19_test1.txt");

        assert_eq!(result, "The total number of possibilities is 167409079868000")
    }
}

pub fn part1(path: &str) -> String {
    let mut lines = utilities::string_iterator(path);
    let workflows = Workflows::read(&mut lines);
    let parts: Vec<Part> = lines
        .map(| txt | Part::read(&txt))
        .collect();
    let total = workflows.assess_parts(&parts);

    format!("The sum of ratings is {total}")
}

pub fn part2(path: &str) -> String {
    let mut lines = utilities::string_iterator(path);
    let workflows = Workflows::read(&mut lines);
    let total = workflows.count_valid();

    format!("The total number of possibilities is {total}")
}

struct Workflows {
    index: HashMap<String, Workflow>
}

impl Workflows {
    fn read(lines: & mut impl Iterator<Item = String>) -> Self {
        let mut line = lines.next().unwrap();
        let mut index: HashMap<String, Workflow> = HashMap::new();

        while line != "" {
            let mut line_parts = line.split('{');
            let name = String::from(line_parts.next().unwrap());
            let workflow_txt = line_parts.next().unwrap().strip_suffix('}').unwrap();
            let workflow = Workflow::read(workflow_txt);
            index.insert(name, workflow);
            line = lines.next().unwrap();
        }

        Self {index }
    }

    fn assess_parts(& self, parts: &[Part]) -> u32 {
        parts.iter()
            .filter(| part | self.assess_part(part))
            .fold(
                0u32,
                | total, part | total + part.rating_sum()
            )
    }

    fn assess_part(& self, part: &Part) -> bool {
        let mut start: &str = &"in";
        let mut end: &str = self.index[start].apply(part);
        while end != "A" && end != "R" {
            // println!("{end}");
            start = end;
            end = self.index[start].apply(part);
        }

        end == "A"
    }

    fn count_valid(& self) -> u64 {
        let mut stack = vec![("in", ValidRanges::new())];
        let mut valid = 0u64;

        while let Some((start, mut valid_ranges)) = stack.pop() {
            let workflow = &self.index[start];
            for rule in &workflow.rules {
                let test = &rule.test;
                let destination = &rule.destination;
                if test.is_end() {
                    if destination == "A" {
                        valid += valid_ranges.permutations();
                    } else if destination != "R" {
                        // This could be a move, strictly speaking, because `End` is always the last entry,
                        // but as the code is now, the compiler has no way of guaranteeing that.
                        // I suppose I could put a `break` after the `if` block.
                        stack.push((destination, valid_ranges.clone()));
                    }
                } else {
                    let mut branch = valid_ranges.clone();
                    branch.update(test);
                    if destination == "A" {
                        valid += branch.permutations();
                    } else if branch.permutations() > 0 && destination != "R" {
                        stack.push((destination, branch));
                    }

                    valid_ranges.update_complement(test);
                }
            }
        }

        valid
    }

}

struct Workflow {
    rules: Vec<Rule>
}

impl Workflow {
    fn read(txt: &str) -> Self {
        let rules_txt = txt.split(',');
        let rules = rules_txt
            .map(| rule | Rule::read(rule))
            .collect();

        Self { rules }
    }
    
    fn apply(& self, part: &Part) -> &str {
        for rule in &self.rules {
            if let Some(destination) = rule.apply(part) {
                return destination;
            }
        }
        panic!("Did not find a destination")
    }
}

#[derive(Debug)]
struct Rule {
    test: Test,
    destination: String
}

impl Rule {
    fn read(txt: &str) -> Self {
        if txt.contains(':') {
            let mut parts = txt.split(':');
            let test_part = parts.next().unwrap();
            let dest_part = parts.next().unwrap();
            Rule {
                test: Test::parse(test_part),
                destination: String::from(dest_part)
            }
        } else {
            Rule {
                test: Test::End,
                destination: String::from(txt)
            }
        }
    }

    fn apply(& self, part: &Part) -> Option<&str> {
        if self.test.apply(part) {
            Some(&self.destination)
        } else {
            None
        }
    }
}

#[derive(Debug)]
enum Test {
    X(Condition),
    M(Condition),
    A(Condition),
    S(Condition),
    End
}

impl Test {
    fn parse(txt: &str) -> Self {
        let mut chs = txt.chars();
        let rating = chs.next().unwrap();
        let operator = chs.next().unwrap();
        let val = chs.fold(
            0u32,
            | val, ch | val*10 + ch.to_digit(10).unwrap()
        );
        let condition = if operator == '>' {
            Condition::GT(val)
        } else {
            Condition::LT(val)
        };

        match rating {
            'x' => Self::X(condition),
            'm' => Self::M(condition),
            'a' => Self::A(condition),
            's' => Self::S(condition),
            _ => panic!("Couldn't find valid rating in test string")
        }
    }
    
    fn apply(& self, part: &Part) -> bool {
        match self {
            Test::X(condition) => {
                condition.apply(part.x)
            },
            Test::M(condition) => {
                condition.apply(part.m)
            },
            Test::A(condition) => {
                condition.apply(part.a)
            },
            Test::S(condition) => {
                condition.apply(part.s)
            },
            Test::End => true
        }
    }

    fn is_end(& self) -> bool {
        match self {
            Self::End => true,
            _ => false
        }
    }
}
    
#[derive(Debug)]
enum Condition {
    LT(u32),
    GT(u32)
}

impl Condition {
    fn apply(& self, other_val: u32) -> bool {
        match self {
            Self::LT(val) => other_val < *val,
            Self::GT(val) => other_val > *val
        }
    }
}

struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32
}

impl Part {
    fn read(txt: &str) -> Self {
        let mut x = 0u32;
        let mut m = 0u32;
        let mut a = 0u32;
        let mut s = 0u32;
        // println!("{txt}");

        let ratings = txt
            .split(',')
            .map(
                | st | {
                    let mut chs = st.chars()
                        .filter(
                            | ch | {
                                *ch != '{' && 
                                *ch != '=' &&
                                *ch != '}'
                            }
                        );
                    let letter = chs.next().unwrap();
                    let value = chs.fold(
                        0u32,
                        | total, ch | {
                            total * 10 + ch.to_digit(10).unwrap()
                        }
                    );
                    (letter, value)
                }
            );
        for (var, val) in ratings {
            match var {
                'x' => {x = val},
                'm' => {m = val},
                'a' => {a = val},
                's' => {s = val},
                _ => {
                    println!("{var}");
                    panic!("Read invalid rating name");
                }
            }
        }

        Self { x, m, a, s }
    }

    fn rating_sum(& self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Clone, Debug)]
struct ValidRanges {
    rating_ranges: [(u32, u32); 4]
}

impl ValidRanges {
    fn new() -> Self {
        let full_range = (1u32, 4000u32);
        let rating_ranges = [full_range; 4];
        Self { rating_ranges }
    }

    fn update(& mut self, test: &Test) {
        match test {
            Test::X(condition) => {
                let index = 0;
                let (current_bottom, current_top) = self.rating_ranges[index];
                let new_range = match condition {
                    Condition::GT(val) => {
                        let new_bottom = current_bottom.max(*val + 1);
                        (new_bottom, current_top)
                    },
                    Condition::LT(val) => {
                        let new_top = current_top.min(*val - 1);
                        (current_bottom, new_top)
                    }
                };
                self.rating_ranges[index] = new_range;
            },
            Test::M(condition) => {
                let index = 1;
                let (current_bottom, current_top) = self.rating_ranges[index];
                let new_range = match condition {
                    Condition::GT(val) => {
                        let new_bottom = current_bottom.max(*val + 1);
                        (new_bottom, current_top)
                    },
                    Condition::LT(val) => {
                        let new_top = current_top.min(*val - 1);
                        (current_bottom, new_top)
                    }
                };
                self.rating_ranges[index] = new_range;
            },
            Test::A(condition) => {
                let index = 2;
                let (current_bottom, current_top) = self.rating_ranges[index];
                let new_range = match condition {
                    Condition::GT(val) => {
                        let new_bottom = current_bottom.max(*val + 1);
                        (new_bottom, current_top)
                    },
                    Condition::LT(val) => {
                        let new_top = current_top.min(*val - 1);
                        (current_bottom, new_top)
                    }
                };
                self.rating_ranges[index] = new_range;
            },
            Test::S(condition) => {
                let index = 3;
                let (current_bottom, current_top) = self.rating_ranges[index];
                let new_range = match condition {
                    Condition::GT(val) => {
                        let new_bottom = current_bottom.max(*val + 1);
                        (new_bottom, current_top)
                    },
                    Condition::LT(val) => {
                        let new_top = current_top.min(*val - 1);
                        (current_bottom, new_top)
                    }
                };
                self.rating_ranges[index] = new_range;
            },
            Test::End => ()
        }
    }

    fn update_complement(& mut self, test: &Test) {
        match test {
            Test::X(condition) => {
                let index = 0;
                let (current_bottom, current_top) = self.rating_ranges[index];
                let new_range = match condition {
                    Condition::LT(val) => {
                        let new_bottom = current_bottom.max(*val);
                        (new_bottom, current_top)
                    },
                    Condition::GT(val) => {
                        let new_top = current_top.min(*val);
                        (current_bottom, new_top)
                    }
                };
                self.rating_ranges[index] = new_range;
            },
            Test::M(condition) => {
                let index = 1;
                let (current_bottom, current_top) = self.rating_ranges[index];
                let new_range = match condition {
                    Condition::LT(val) => {
                        let new_bottom = current_bottom.max(*val);
                        (new_bottom, current_top)
                    },
                    Condition::GT(val) => {
                        let new_top = current_top.min(*val);
                        (current_bottom, new_top)
                    }
                };
                self.rating_ranges[index] = new_range;
            },
            Test::A(condition) => {
                let index = 2;
                let (current_bottom, current_top) = self.rating_ranges[index];
                let new_range = match condition {
                    Condition::LT(val) => {
                        let new_bottom = current_bottom.max(*val);
                        (new_bottom, current_top)
                    },
                    Condition::GT(val) => {
                        let new_top = current_top.min(*val);
                        (current_bottom, new_top)
                    }
                };
                self.rating_ranges[index] = new_range;
            },
            Test::S(condition) => {
                let index = 3;
                let (current_bottom, current_top) = self.rating_ranges[index];
                let new_range = match condition {
                    Condition::LT(val) => {
                        let new_bottom = current_bottom.max(*val);
                        (new_bottom, current_top)
                    },
                    Condition::GT(val) => {
                        let new_top = current_top.min(*val);
                        (current_bottom, new_top)
                    }
                };
                self.rating_ranges[index] = new_range;
            },
            Test::End => ()
        }
    }
    
    fn permutations(& self) -> u64 {
        self.rating_ranges
            .iter()
            .map(
                | (bottom, top) | {
                    if *bottom > *top {
                        0u64
                    } else {
                        *top as u64 - *bottom as u64 + 1
                    }
                }
            )
            .product()
    }
}
