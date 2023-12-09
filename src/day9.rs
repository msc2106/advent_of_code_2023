use crate::utilities;

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn part1_test() {
        let result = part1("./input/day9_test1.txt");

        assert_eq!(result, "The sum of forecast values is 114");
    }

    #[test]
    fn part2_test() {
        let result = part2("./input/day9_test1.txt");

        assert_eq!(result, "The sum of backcast values is 2");
    }
}

pub fn part1(path: &str) -> String {
    let lines = utilities::string_iterator(path);
    let forecasters = lines.map(|l| Forecaster::read(&l));
    let forecasts = forecasters.map(|f| f.forecast());
    let sum_of_forecasts: i32 = forecasts.sum();

    format!("The sum of forecast values is {sum_of_forecasts}")
}

pub fn part2(path: &str) -> String {
    let lines = utilities::string_iterator(path);
    let forecasters = lines.map(|l| Forecaster::read(&l));
    let backcasts = forecasters.map(|f| f.backcast());
    let sum_of_backcasts: i32 = backcasts.sum();

    format!("The sum of backcast values is {sum_of_backcasts}")
}

struct Forecaster {
    sequence: Vec<i32>
}

impl Forecaster {
    fn read(line: &str) -> Self {
        let sequence = line
            .split(' ')
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();
        
        Self { sequence }
    }

    /// forecasts one additional value
    fn forecast(&self) -> i32 {
        let mut differences: Vec<Vec<i32>> = Vec::new();
        let mut current_sequence = &self.sequence;

        loop {
            let difference = Self::difference(current_sequence);
            if difference.iter().all(|v| *v == 0) {
                break;
            } else {
                differences.push(difference);
                current_sequence = differences.last().unwrap();
            }
        }
        
        let mut forecast_val = 0;

        for difference in differences.iter().rev() {
            forecast_val += difference.last().unwrap();
        }
        // println!("{differences:?} \n {forecast_val}");
        forecast_val + self.sequence.last().unwrap()
    }

    /// backcasts one additional value
    fn backcast(&self) -> i32 {
        let mut differences: Vec<Vec<i32>> = Vec::new();
        let mut current_sequence = &self.sequence;

        loop {
            let difference = Self::difference(current_sequence);
            if difference.iter().all(|v| *v == 0) {
                break;
            } else {
                differences.push(difference);
                current_sequence = differences.last().unwrap();
            }
        }
        
        let mut backcast_val = 0;

        for difference in differences.iter().rev() {
            backcast_val = difference[0] - backcast_val;
        }
        // println!("{differences:?} \n {backcast_val}");
        self.sequence[0] - backcast_val
    }

    fn difference(sequence: &Vec<i32>) -> Vec<i32> {
        let base = sequence.iter();
        let offset = sequence.iter().skip(1);
        let differences: Vec<i32> = base.zip(offset)
            .map(|(first, second)| second - first)
            .collect();
        differences
    }
    
}