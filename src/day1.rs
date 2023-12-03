use crate::utilities;
use std::{thread, str::Chars};

pub fn part1() -> String {
    let mut lines = utilities::lines_from_file("input/1-1.txt");
    let mut total:u32 = 0;
    while let Some(Ok(line)) = lines.next() {
        // println!("{}", &line);
        let chars = line.chars();
        let rev_chars = line.chars().rev();
        let first_num = first_number(chars).expect("Couldn't find a first number");
        let last_num = first_number(rev_chars).expect("Couldn't find a last number");
        total += first_num*10 + last_num;
    }
    
    format!("Sum: {total}")
}

fn first_number(chars: impl Iterator<Item = char>) -> Option<u32> {
    for ch in chars {
        if let Some(num) = ch.to_digit(10) {
            return Some(num);
        }
    }
    return None;
}

pub fn part2() -> String {
    let mut lines = utilities::lines_from_file("input/1-1.txt");
    let mut total:u32 = 0;
    let mut threads = Vec::new();
    while let Some(Ok(line)) = lines.next() {
        threads.push(thread::spawn(move || {process_line(line.to_ascii_lowercase())}));
    }
    for handle in threads {
        if let Ok(val) = handle.join() {
            total += val;
        } else {
            panic!("failed to get a valid response from the thread");
        }
    }
    format!("{total}")
}

// zero
// one
// two
// three
// four
// five
// six
// seven
// eight
// nine
fn process_line(line:String) -> u32 {
    let line_rev: String = line.chars().rev().collect();
    let numstr: [&str; 10] = [
        "zero",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine"
    ];
    let num_key_rev: Vec<(String, u32)> = numstr
                                            .iter()
                                            .map(|s| {
                                                s.chars().rev().skip(1).collect::<String>()
                                            })
                                            .zip(0..10)
                                            .collect();
    let num_key: Vec<(String, u32)> = numstr
                                        .iter()
                                        .map(|s| {
                                            s.chars().skip(1).collect::<String>()
                                        })
                                        .zip(0..10u32)
                                        .collect();
    let first = extract_digit(line.chars(), &num_key).expect("Failed to find first digit");
    let last = extract_digit(line_rev.chars(), &num_key_rev).expect("Failed to find last digit");
    
    // println!("{}: {}{}", line, first, last);
    first * 10 + last

    // while let Some(ch) = chars.next() {
    //     if let Some(num) = ch.to_digit(10) {
    //         first = num;
    //         break;
    //     } else {
    //         match ch {
    //             'z' => {
    //                 if chars.as_str().starts_with("ero") {
    //                     first = 0;
    //                     break;
    //                 }
    //             },
    //             'o' => {
    //                 if chars.as_str().starts_with("ne") {
    //                     first = 1;
    //                     break;
    //                 }
    //             },
    //             't' => {
    //                 if chars.as_str().starts_with("wo") {
    //                     first = 2;
    //                     break;
    //                 } else if chars.as_str().starts_with("hree") {
    //                     first = 3;
    //                     break;
    //                 }
    //             },
    //             'f' => {
    //                 if chars.as_str().starts_with("our") {
    //                     first = 4;
    //                     break;
    //                 } else if chars.as_str().starts_with("ive") {
    //                     first = 5;
    //                     break;
    //                 }
    //             },
    //             's' => {
    //                 if chars.as_str().starts_with("ix") {
    //                     first = 6;
    //                     break;
    //                 } else if chars.as_str().starts_with("even") {
    //                     first = 7;
    //                     break;
    //                 }
    //             },
    //             'e' => {
    //                 if chars.as_str().starts_with("ight") {
    //                     first = 8;
    //                     break;
    //                 }
    //             },
    //             'n' => {
    //                 if chars.as_str().starts_with("ine") {
    //                     first = 9;
    //                     break;
    //                 }
    //             }
    //             _ => ()
    //         }
    //     }
    // }


}

fn extract_digit(mut chars: Chars, key: &[(String, u32)]) -> Option<u32> {
    while let Some(ch) = chars.next() {
        if let Some(v) = ch.to_digit(10) {
            return Some(v)
        }
        for (s, v) in key {
            if chars.as_str().starts_with(s) {
                return Some(*v);
                
            }
        }
    }
    None
}
