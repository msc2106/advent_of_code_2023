use std::env;

mod utilities;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;

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
            "8-1" => day8::part1("./input/day8.txt"),
            "8-2" => day8::part2("./input/day8.txt"),
            "9-1" => day9::part1("./input/day9.txt"),
            "9-2" => day9::part2("./input/day9.txt"),
            "10-1" => day10::part1("./input/day10.txt"),
            "10-2" => day10::part2("./input/day10.txt"),
            "11-1" => day11::part1("./input/day11.txt"),
            "11-2" => day11::part2("./input/day11.txt"),
            "12-1" => day12::part1("./input/day12.txt"),
            "12-2" => day12::part2("./input/day12.txt"),
            "13-1" => day13::part1("./input/day13.txt"),
            "13-2" => day13::part2("./input/day13.txt"),
            "14-1" => day14::part1("./input/day14.txt"),
            "14-2" => day14::part2("./input/day14.txt"),
            "15-1" => day15::part1("./input/day15.txt"),
            "15-2" => day15::part2("./input/day15.txt"),
            // "12-1" => day12::part1("./input/day12.txt"),
            // "12-2" => day12::part2("./input/day12.txt"),
            // "13-1" => day13::part1("./input/day13.txt"),
            // "13-2" => day13::part2("./input/day13.txt"),
            _ => String::from("Problem not implemented")
        }
    } else {
        String::from("No argument")
    };

    println!("{result}");
}
