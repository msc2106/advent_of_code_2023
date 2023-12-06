use crate::utilities;
use std::cmp::Ordering;
use std::ops::Range;

#[cfg(test)]
mod testing {
    use super::*;
    
    #[test]
    fn part1_test() {
        let output = part1("./input/day5_test1.txt");
        assert_eq!(output, "The minimum location number is 35");
    }

    #[test]
    fn part2_test() {
        let output = part2("./input/day5_test1.txt");
        assert_eq!(output, "The minimum location number is 46");  
    }

    #[test]
    fn part2_naive_test() {
        let output = _part2("./input/day5_test1.txt");
        assert_eq!(output, "The minimum location number is 46");  
    }
}

pub fn part1(path: &str) -> String {
    let mut lines = utilities::string_iterator(path);
    let seeds: Vec<i64> = lines.next()
        .expect("Can't read first line")
        .split(' ')
        .skip(1)
        .filter_map(|s| s.parse::<i64>().ok())
        .collect();
    _ = lines.next();
    
    let mut mappers: Vec<Mapper> = Vec::new();
    while let Some(line) = lines.next() {
        if line.ends_with("map:") {
            mappers.push(Mapper::read_mapping(& mut lines));
        }
    }

    // println!("Mappers: {mappers:?}");
    // println!("Start values: {seeds:?}");
    // println!("First conversion {:?}", seeds.iter().map(|s| mappers[0].convert(*s)).collect::<Vec<i64>>());

    let end_vals: Vec<i64> = mappers
        .iter()
        .fold(
            seeds, 
            |input_vals: Vec<i64>, mapper| { 
                input_vals
                    .iter()
                    .map(|v| mapper.convert(*v))
                    .collect()
            }
        );
    
    // println!("Ending values: {end_vals:?}");
    let output_val = end_vals.iter().min().expect("Find min failed");

    format!("The minimum location number is {output_val}")
}

/// Much faster approach that iterates through target values from 0
/// Still takes several second on debug (much less on release)
/// Efficient method would instead use end-point ranges rather than iterating one value at a time
pub fn part2(path: &str) -> String {
    let mut lines = utilities::string_iterator(path);
    let seed_ranges = RangeList::read(&lines.next().expect("Can't read first line"));
    
    _ = lines.next();
    
    let mut mappers: Vec<Mapper> = Vec::new();
    while let Some(line) = lines.next() {
        if line.ends_with("map:") {
            mappers.push(Mapper::read_mapping_reverse(& mut lines));
        }
    }

    let mut min_val:Option<i64> = None;
    let mut target = 0i64;
    while min_val.is_none() {
        let candidate = mappers.iter().rev().fold(target, |in_val, mapper| mapper.convert(in_val));
        if seed_ranges.contains(candidate) {
            min_val = Some(target);
        } else {
            target += 1;
        }
    }

    let output_val = min_val.expect("Failed to find minimum");

    format!("The minimum location number is {output_val}")
}

/// Naive implementation that iterates through all seed values.
/// Gets correct answer (`31161857`) but takes a very long time.
pub fn _part2(path: &str) -> String {
    let mut lines = utilities::string_iterator(path);
    let seed_ranges = _RangeMaker::_read(&lines.next().expect("Can't read first line"));
    
    _ = lines.next();
    
    let mut mappers: Vec<Mapper> = Vec::new();
    while let Some(line) = lines.next() {
        if line.ends_with("map:") {
            mappers.push(Mapper::read_mapping(& mut lines));
        }
    }

    // println!("Mappers: {mappers:?}");
    // println!("Start values: {seeds:?}");
    // println!("First conversion {:?}", seeds.iter().map(|s| mappers[0].convert(*s)).collect::<Vec<i64>>());

    let output_val = seed_ranges
        .map(|start| { 
                mappers
                    .iter()
                    .fold(start, |in_val, mapper| mapper.convert(in_val))
            })
        .min()
        .expect("Couldn't find minimum");

    format!("The minimum location number is {output_val}")
}

#[derive(Debug)]
struct Mapper {
    rules: Vec<MapperElement>
}

impl Mapper {
    /// Iterates through given lines and generates a mapper from them. The returned iterator will either be exhausted or have just outputed a blank line (i.e. next should be a header line).
    fn read_mapping(lines: & mut impl Iterator<Item = String>) -> Self {
        let mut rules = Vec::new();
        loop {
            if let Some(line) = lines.next() {
                if line == "" {
                    break;
                } else {
                    let new_elem = MapperElement::read_line(&line);
                    rules.push(new_elem);
                }
            } else {
                break;
            }
        }

        rules.sort();

        Self { rules }
    }

    fn read_mapping_reverse(lines: & mut impl Iterator<Item = String>) -> Self {
        let mut rules = Vec::new();
        loop {
            if let Some(line) = lines.next() {
                if line == "" {
                    break;
                } else {
                    let new_elem = MapperElement::read_line_reverse(&line);
                    rules.push(new_elem);
                }
            } else {
                break;
            }
        }

        rules.sort();

        Self { rules }
    }

    fn convert(&self, input_val: i64) -> i64 {
        let mut offset = 0i64;
        for rule in &self.rules {
            if input_val >= rule.origin_start && input_val <= rule.origin_end {
                offset = rule.offset;
                break;
            } else if rule.origin_start > input_val {
                break;
            }
        }

        input_val + offset
    }
}

#[derive(Debug)]
struct MapperElement {
    origin_start: i64,
    origin_end: i64,
    offset: i64
}

impl MapperElement {
    fn read_line(line: &str) -> Self {
        let vals: Vec<i64> = line
            .split(' ')
            .filter_map(|s| s.parse::<i64>().ok())
            .collect();
        if vals.len() != 3 {
            panic!("Read a line without 3 numbers");
        }
        let origin_start = vals[1];
        let origin_end = origin_start + vals[2] - 1;
        let offset = vals[0] - origin_start;

        Self {origin_start, origin_end, offset}
    }

    fn read_line_reverse(line: &str) -> Self {
        let vals: Vec<i64> = line
            .split(' ')
            .filter_map(|s| s.parse::<i64>().ok())
            .collect();
        if vals.len() != 3 {
            panic!("Read a line without 3 numbers");
        }
        let origin_start = vals[0];
        let origin_end = origin_start + vals[2] - 1;
        let offset = vals[1] - origin_start;

        Self {origin_start, origin_end, offset}
    }
}

impl Ord for MapperElement {
    fn cmp(&self, other: &Self) -> Ordering {
        self.origin_start.cmp(&other.origin_start)
    }
}

impl PartialOrd for MapperElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for MapperElement {
    fn eq(&self, other: &Self) -> bool {
        self.origin_start == other.origin_start
    }
}

impl Eq for MapperElement {
    // PartialEq is all that is actually necessary
}

struct RangeList {
    ranges: Vec<(i64, i64)>
}

/// Ordered list of ranges
/// Internally ranges are defined by a start and inclusive endpoint
impl RangeList {
    fn read(num_str: &str) -> Self {
        let mut nums = num_str
            .split(' ')
            .filter_map(|s| s.parse::<i64>().ok());

        let mut ranges: Vec<(i64, i64)> = Vec::new();
        while let Some(first_num) = nums.next() {
            let second_num = nums.next().expect("Odd number of values in seed ranges");
            ranges.push((first_num, first_num+second_num-1));
        }
        
        ranges.sort_by_key(|t| t.0 );

        Self { ranges }
    }

    fn contains(&self, other:i64) -> bool {
        for (start, end) in &self.ranges {
            if other >= *start && other <= *end {
                return true;
            } else if *start > other {
                return false;
            }
        }
        
        false
    }
}
/// Reads a set of ranges defined by start point and length and iterates through values.
/// Order is increasing within range, from last range read to first.
/// Internally, ranges are defined by a start and non-inclusive endpoint (like `start..end`)
struct _RangeMaker {
    ranges: Vec<(i64, i64)>,
    current_range: Option<Range<i64>>,
}

impl _RangeMaker{
    fn _read(num_str: &str) -> Self {
        let mut nums = num_str
            .split(' ')
            .filter_map(|s| s.parse::<i64>().ok());

        let mut ranges: Vec<(i64, i64)> = Vec::new();
        while let Some(first_num) = nums.next() {
            let second_num = nums.next().expect("Odd number of values in seed ranges");
            ranges.push((first_num, first_num+second_num));
        }
        let (start, end) = ranges.pop().expect("Read list of ranges empty");
        let current_range = Some(start..end);
        Self { ranges, current_range}
    }
}

impl Iterator for _RangeMaker {
    type Item = i64;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        if let Some(range) = & mut self.current_range {
            if let Some(next) = range.next() {
                Some(next)
            } else if let Some((start, end)) = self.ranges.pop() {
                let mut new_range = start..end;
                let next = new_range.next();
                self.current_range = Some(new_range);
                next
            } else {
                self.current_range = None;
                None
            }
        } else {
            None
        }
    }
}
