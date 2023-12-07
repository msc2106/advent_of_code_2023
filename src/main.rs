use std::env;

mod utilities;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() {
    let mut args = env::args();
    args.next();
    let result = if let Some(problem) = args.next()  {
        match problem.as_str() {
            "1-1" => day1::part1(),
            "1-2" => day1::part2(),
            "2-1" => day2::part1(),
            "2-2" => day2::part2(),
            "3-1" => day3::part1("./input/day3.txt"),
            "3-2" => day3::part2("./input/day3.txt"),
            "4-1" => day4::part1("./input/day4.txt"),
            "4-2" => day4::part2("./input/day4.txt"),
            "5-1" => day5::part1("./input/day5.txt"),
            "5-2" => day5::part2("./input/day5.txt"),
            "6-1" => day6::part1("./input/day6.txt"),
            "6-2" => day6::part2("./input/day6.txt"),
            "7-1" => day7::part1("./input/day7.txt"),
            "7-2" => day7::part2("./input/day7.txt"),
            // "8-1" => day8::part1(),
            // "8-2" => day8::part2(),
            _ => String::from("Problem not implemented")
        }
    } else {
        String::from("No argument")
    };

    println!("{result}");
}
