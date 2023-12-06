use crate::utilities;

#[cfg(test)]
mod testing {
    use super::*;
    
    #[test]
    fn part1_test() {
        let output = part1("./input/day6_test1.txt");
        assert_eq!(output, "The product is 288");
    }

    #[test]
    fn part2_test() {
        let output = part2("./input/day6_test1.txt");
        assert_eq!(output, "There are 71503 ways to win");  
    }
}

pub fn part1(path: &str) -> String {
    let lines:Vec<String> = utilities::string_iterator(path).collect();
    if lines.len() != 2 {
        panic!("Wrong number of lines");
    }
    let times = lines[0].split(' ').filter_map(|s| s.parse::<u64>().ok());
    let distances = lines[1].split(' ').filter_map(|s| s.parse::<u64>().ok());

    let result:u32 = times
        .zip(distances)
        .map(|(t, d)| solve(t, d))
        .product();

    format!("The product is {result}")
}

pub fn part2(path: &str) -> String {
    let lines:Vec<String> = utilities::string_iterator(path).collect();
    if lines.len() != 2 {
        panic!("Wrong number of lines");
    }
    let time = reduce_digits(&lines[0]);
    let distance = reduce_digits(&lines[1]);

    let result:u32 = solve(time, distance);

    format!("There are {result} ways to win") 
}

fn reduce_digits(line: &str) -> u64 {
    line
        .chars()
        .filter_map(|ch| ch.to_digit(10))
        .fold(0u64,|l, r| l * 10u64 + r as u64)
}

/// Return the number of integer solutions of p for `p` <sup>`2`</sup> `- t*p + d < 0` given `t` and `d`
/// Using the quadratic formula the solutions are in `(t - sqrt(t`<sup>`2`</sup>` - 4*d))/2 < p < (t + sqrt(t`<sup>`2`</sup>`-4*d))/2`
fn solve(t: u64, d: u64) -> u32 {
    let float_t = t as f64;
    let float_d = d as f64;

    // Make sure there are real solutions
    let discrim = (float_t.powi(2) - 4.0 * float_d).sqrt();
    if discrim.is_nan() {
        return 0;
    }

    // All real solutions are positive
    let lower_crit = (float_t - discrim)/2.0;
    let upper_crit = (float_t + discrim)/2.0;

    // println!("t={t}, d={d}, solutions: {lower_crit} < p < {upper_crit} ");

    // need to handles cases where critical values fall on right on an integer
    let lower_bound = if lower_crit != lower_crit.ceil() {
        lower_crit.ceil() as u32
    } else {
        lower_crit as u32 + 1 
    };
    let upper_bound = if upper_crit != upper_crit.floor() {
        upper_crit.floor() as u32
    } else {
        upper_crit as u32 - 1 
    };

    // println!("Number of integer solutions: {}", upper_bound - lower_bound + 1);

    upper_bound - lower_bound + 1
}