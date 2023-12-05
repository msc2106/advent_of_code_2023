use crate::utilities;

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn part1_test() {
        let result = part1("./input/day4_test1.txt");
        assert_eq!(result, "The points total is 13");
    }

    #[test]
    fn part2_test() {
        let result = part2("./input/day4_test1.txt");
        assert_eq!(result, "The final total of cards is 30");
    }
}

pub fn part1(path: &str) -> String {
    let lines = utilities::string_iterator(path);
    let mut cards: Vec<Card> = Vec::new();
    for line in lines {
        cards.push(Card::read_line(&line));
    }

    let total:u32 = cards
        .iter()
        .map(|c| c.value())
        .sum();

    format!("The points total is {total}")
}

pub fn part2(path: &str) -> String {
    let lines = utilities::string_iterator(path);
    let mut cards: Vec<Card> = Vec::new();
    for line in lines {
        cards.push(Card::read_line(&line));
    }
    let mut card_counts:Vec<u32> = vec![1; cards.len()];
    for i in 0..cards.len() {
        let won = cards[i].winning_count();
        for j in i + 1 .. i + 1 + won as usize {
            card_counts[j] += card_counts[i];
        }
    }

    let total: u32 = card_counts.iter().sum();
    
    format!("The final total of cards is {total}")
}

#[derive(Debug)]
struct Card {
    winning: Vec<u32>,
    nums: Vec<u32>
}

impl Card {
    fn read_line(line:&str) -> Self {
        // let mut winning = Vec::new();
        // let mut nums = Vec::new();

        let mut card_text = line
            .split(": ")
            .skip(1)
            .next()
            .expect("Couldn't extract numbers from line")
            .split(" | ");
        
        let winning = card_text
            .next()
            .expect("Couldn't extract winning numbers part")
            .split(' ')
            .filter_map(|n| n.parse::<u32>().ok())
            .collect();

        let nums = card_text
        .next()
        .expect("Couldn't extract numbers part")
        .split(' ')
        .filter_map(|n| n.parse::<u32>().ok())
        .collect();

        Self {winning, nums}
    }

    fn value(&self) -> u32 {
        let winning_count = self.winning_count();

        let score:u32 = if winning_count < 2 {
            winning_count
        } else {
            (0..winning_count)
            .skip(1)
            .fold(1, |l, _| {l*2})
        };
        
        // println!("{self:?}");
        // println!("{winning_count} winning numbers, score {score}");
        
        score
    }

    fn winning_count(&self) -> u32 {
        let mut winning_count = 0u32;
        for winning_num in &self.winning {
            if self.nums.contains(winning_num) {
                winning_count += 1;
            }
        }
        winning_count
    }
}

