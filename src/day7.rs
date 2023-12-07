use crate::utilities;
use std::{collections::HashMap, cmp::Ordering, fmt::Debug, hash::Hash};

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn part1_test() {
        let result = part1("./input/day7_test1.txt");

        assert_eq!(result, "The total winnings are 6440");
    }

    #[test]
    fn part2_test() {
        let result = part2("./input/day7_test1.txt");

        assert_eq!(result, "The total winnings are 5905");
    }
}

pub fn part1(path: &str) -> String {
    let lines = utilities::string_iterator(path);
    let mut hands: Vec<Hand<Card>> = lines.map(|l| Hand::<Card>::read(&l)).collect();
    hands.sort();
    let rank = 1..hands.len()+1;

    // println!("{hands:?}");

    let total:u32 = hands
        .iter()
        .zip(rank)
        .fold(
                0u32, 
                |prev_total, (hand, rank)| {prev_total + hand.bid * rank as u32}
            );

    format!("The total winnings are {total}")
}

pub fn part2(path: &str) -> String {
    let lines = utilities::string_iterator(path);
    let mut hands: Vec<Hand<JokerCard>> = lines.map(|l| Hand::<JokerCard>::read(&l)).collect();
    hands.sort();
    let rank = 1..hands.len()+1;

    // println!("{hands:?}");

    let total:u32 = hands
        .iter()
        .zip(rank)
        .fold(
                0u32, 
                |prev_total, (hand, rank)| {prev_total + hand.bid * rank as u32}
            );

    format!("The total winnings are {total}")
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandType {
    High = 0,
    Pair = 1,
    TwoPair = 2,
    Triple = 3,
    Full = 4,
    Four = 5,
    Five = 6
}

#[derive(Debug, Eq)]
struct Hand <T:CardSet> {
    cards: [T; 5],
    hand_type: HandType,
    bid: u32
}

impl<T:CardSet> Hand<T> {
    fn _read(line: &str) -> ([T;5], u32) {
        let mut split = line.split(' ');
        let cards: [T; 5] = split
            .next()
            .expect("Couldn't find hand part of line")
            .chars()
            .filter_map(|s| T::try_from(s).ok())
            .collect::<Vec<T>>()
            .try_into()
            .expect("Couldn't parse hand");
        
        // let hand_type = Hand::categorize(&cards);
        
        let bid: u32 = split
            .next()
            .expect("Couldn't find bid part of line")
            .parse()
            .expect("Couldn't parse bid");

        (cards, bid)
    }
}

impl Hand<Card> {
    fn read(line: &str) -> Self {
        let (cards, bid) = Self::_read(line);
        let hand_type = Self::categorize(&cards);
        Self { cards, hand_type, bid }
    }
    
    fn categorize(cards: &[Card; 5]) -> HandType {
        let mut card_counts: HashMap<Card, u8> = HashMap::new();
        for card in cards {
            if let Some(val) = card_counts.get(card) {
                card_counts.insert(*card, *val+1);
            } else {
                card_counts.insert(*card, 1);
            }
        }

        // println!("{card_counts:?}");

        match card_counts.len() {
            1 => HandType::Five,
            2 => {
                if *card_counts.values().max().unwrap() == 4u8 {
                    HandType::Four
                } else {
                    HandType::Full
                }
            },
            3 => {
                if *card_counts.values().max().unwrap() == 3u8 {
                    HandType::Triple
                } else {
                    HandType::TwoPair
                }
            },
            4 => HandType::Pair,
            5 => HandType::High,
            _ => panic!("Couldn't categorize hand")
        }
    }
}

impl Hand<JokerCard> {
    fn read(line: &str) -> Self {
        let (cards, bid) = Self::_read(line);
        let hand_type = Self::categorize(&cards);
        Self { cards, hand_type, bid }
    }

    fn categorize(cards: &[JokerCard; 5]) -> HandType {
        let mut card_counts: HashMap<JokerCard, u8> = HashMap::new();
        for card in cards {
            if let Some(val) = card_counts.get(card) {
                card_counts.insert(*card, *val+1);
            } else {
                card_counts.insert(*card, 1);
            }
        }

        // println!("{card_counts:?}");

        let original_type = match card_counts.len() {
            1 => HandType::Five,
            2 => {
                if *card_counts.values().max().unwrap() == 4u8 {
                    HandType::Four
                } else {
                    HandType::Full
                }
            },
            3 => {
                if *card_counts.values().max().unwrap() == 3u8 {
                    HandType::Triple
                } else {
                    HandType::TwoPair
                }
            },
            4 => HandType::Pair,
            5 => HandType::High,
            _ => panic!("Couldn't categorize hand")
        };

        let joker_count = if let Some(cnt) = card_counts.get(&JokerCard::Joker) {
            *cnt
        } else {
            0u8
        };

        match joker_count {
            0 => original_type,
            1 => match original_type {
                HandType::High => HandType::Pair,
                HandType::Pair => HandType::Triple,
                HandType::TwoPair => HandType::Full,
                HandType::Triple => HandType::Four,
                HandType::Four => HandType::Five,
                _ => panic!("Hand type doesn't work with 1 joker")
            },
            2 => match original_type {
                HandType::Pair => HandType::Triple,
                HandType::TwoPair => HandType::Four,
                HandType::Full => HandType::Five,
                _ => panic!("Hand type doesn't work with 2 jokers")
            }
            3 => match original_type {
                HandType::Triple => HandType::Four,
                HandType::Full => HandType::Five,
                _ => panic!("Hand type doesn't work with 3 jokers")
            },
            4 => HandType::Five,
            5 => original_type,
            _ => panic!("Invalid number of jokers")
        }
    }
}

impl<T:CardSet> PartialEq for Hand<T> {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type &&
        self.cards
            .iter()
            .zip(other.cards.iter())
            .all(|(l, r)| l == r)
    }
}

impl<T:CardSet> Ord for Hand<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                let mut cards_cmp = self
                    .cards
                    .iter()
                    .zip(other.cards.iter())
                    .map(|(l, r)| l.cmp(r))
                    .skip_while(|c| c.is_eq());
                if let Some(order) = cards_cmp.next() {
                    order
                } else {
                    Ordering::Equal
                }
            }
        }
    }
}

impl<T:CardSet> PartialOrd for Hand<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

trait CardSet: PartialEq + Eq + PartialOrd + Ord + Debug + Hash + Clone + Copy + TryFrom<char> {}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Clone, Copy)]
enum Card {
    Two = 0,
    Three = 1,
    Four = 2,
    Five = 3,
    Six = 4,
    Seven = 5,
    Eight = 6,
    Nine = 7,
    Ten = 8,
    Jack = 9,
    Queen = 10,
    King = 11,
    Ace = 12
}

impl TryFrom<char> for Card {
    type Error = & 'static str;
    
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            'T' => Ok(Self::Ten),
            'J' => Ok(Self::Jack),
            'Q' => Ok(Self::Queen),
            'K' => Ok(Self::King),
            'A' => Ok(Self::Ace),
            _ => Err("Invalid card")
        }
    }
}

impl CardSet for Card {}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Clone, Copy)]
enum JokerCard {
    Joker = 0,
    Two = 1,
    Three = 2,
    Four = 3,
    Five = 4,
    Six = 5,
    Seven = 6,
    Eight = 7,
    Nine = 8,
    Ten = 9,
    Queen = 10,
    King = 11,
    Ace = 12
}

impl TryFrom<char> for JokerCard {
    type Error = & 'static str;
    
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            'T' => Ok(Self::Ten),
            'J' => Ok(Self::Joker),
            'Q' => Ok(Self::Queen),
            'K' => Ok(Self::King),
            'A' => Ok(Self::Ace),
            _ => Err("Invalid card")
        }
    }
}

impl CardSet for JokerCard {}
