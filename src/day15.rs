use crate::utilities;

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn part1_test1() {
        let result = part1("./input/day15_test1.txt");

        assert_eq!(result, "The total hash value is 1320")
    }

    #[test]
    fn part2_test1() {
        let result = part2("./input/day15_test1.txt");

        assert_eq!(result, "The total focal power is 145")
    }
}

pub fn part1(path: &str) -> String {
    let total: usize = utilities::string_iterator(path)
        .next()
        .unwrap()
        .split(',')
        .map(| s | hash(s))
        .sum();

    format!("The total hash value is {total}")
}

pub fn part2(path: &str) -> String {
    let line = utilities::string_iterator(path).next().unwrap();
    let mut boxes: Vec<Vec<Lens>> = vec![Vec::new(); 256];

    for entry in line.split(',') {
        let mut chs = entry.chars();
        let mut ch = chs.next().unwrap();
        let mut label = String::new();
        while ch.is_alphabetic() {
            label.push(ch);
            ch = chs.next().unwrap();
        }
        let label_hash = hash(&label);
        if ch == '-' {
            let in_box = boxes[label_hash].iter().position(| l | l.label_eq(&label));
            if let Some(index) = in_box {
                _ = boxes[label_hash].remove(index);
            }
        } else {
            let focal_length: u32 = chs.as_str().parse().expect("Couldn't parse focal length");
            let in_box = boxes[label_hash].iter().position(| l | l.label_eq(&label));
            if let Some(index) = in_box {
                boxes[label_hash][index].focal_length = focal_length;
            } else {
                let new_lens = Lens::new(label, focal_length);
                boxes[label_hash].push(new_lens);
            }
            
        }
    }
    let total: usize = boxes.iter()
        .enumerate()
        .map(
            | (boxnum, contents) | {
                (boxnum + 1) * 
                contents
                    .iter()
                    .enumerate()
                    .map( | (position, lens) | (position+1) * lens.focal_length as usize)
                    .sum::<usize>()
            }
        )
        .sum();

    format!("The total focal power is {total}")
}

fn hash(s: &str) -> usize {
    s.as_bytes()
     .iter()
     .fold(
        0usize,
        | total, byte | {
            ((total + *byte as usize) * 17) % 256
        }
     )
}

#[derive(Clone, Debug)]
struct Lens {
    label: String,
    focal_length: u32
}

impl Lens {
    fn new(label: String, focal_length: u32) -> Self {
        Self {label, focal_length}
    }
    
    fn label_eq(&self, other_label: &str) -> bool {
        self.label == *other_label
    }
}