use crate::utilities;

pub fn part1() -> String {
    let mut records:Vec<GameRecord> = Vec::new();
    let lines = utilities::lines_from_file("./input/day2.txt");
    for line in lines {
        if let Ok(line_text) = line {
            records.push(GameRecord::parse(line_text));
        } else {
            panic!("Failed to read a line");
        }
    }

    let maximums = Cubes {red:12, green:13, blue:14};
    let total:u32 = records.iter()
                        .filter(|r| r.possible(&maximums))
                        .map(|r| r.id)
                        .sum();

    // for record in &records {
    //     if record.possible(&maximums) {
    //         total += record.id
    //     }
    // }

    format!("Total of possible game IDs: {}", total)
}

pub fn part2() -> String {
    let mut records:Vec<GameRecord> = Vec::new();
    let lines = utilities::lines_from_file("./input/day2.txt");
    for line in lines {
        if let Ok(line_text) = line {
            records.push(GameRecord::parse(line_text));
        } else {
            panic!("Failed to read a line");
        }
    }
    let total:u32 = records.iter()
                            .map(|r| r.power())
                            .sum();

    format!("Sum of powers: {}", total)
}

struct Cubes {
    green: u32,
    red: u32,
    blue: u32
}

impl Cubes {
    fn parse(record:&str) -> Self {
        let mut green = 0u32;
        let mut red = 0u32;
        let mut blue = 0u32;
        let entries = record.split(',').map(|s| s.trim());
        for entry in entries {
            let mut entry_split = entry.split(' ');
            let val_part = entry_split.next().expect("Couldn't get a value");
            let value:u32 = val_part.parse().expect("Couldn't parse value");
            let color_part = entry_split.next().expect("Couldn't get a color");
            match color_part {
                "blue" => blue = value,
                "red" => red = value,
                "green" => green = value,
                _ => panic!("Unrecognized color: {}", color_part)
            };
        }
        
        Self { green, red, blue }
    }

    fn zeros() -> Self {
        Self { red:0, blue:0, green:0 }
    }

    fn possible(&self, other:&Cubes) -> bool {
        self.green <= other.green && self.red <= other.red && self.blue <= other.blue
    }

    fn max(&self, other:&Cubes) -> Self {
        let blue = if self.blue > other.blue {self.blue} else {other.blue};
        let red = if self.red > other.red {self.red} else {other.red};
        let green = if self.green > other.green {self.green} else {other.green};
        Self{ blue, red, green}
    }

    fn prod(&self) -> u32 {
        self.red * self.blue * self.green
    }
}

struct GameRecord {
    id:u32,
    draws:Vec<Cubes>
}

impl GameRecord {
    fn parse(line:String) -> Self {
        let mut draws:Vec<Cubes> = Vec::new();

        let mut id_split = line.split(": ");
        let id_part = id_split.next().expect("Couldn't split on colon").strip_prefix("Game ").expect("Line did not start with 'Game '");
        let record_part = id_split.next().expect("Couldn't split on colon");
        for draw_record in record_part.split("; ") {
            draws.push(Cubes::parse(draw_record));
        }
        Self { 
            id: id_part.parse().expect("Couldn't parse game ID"), 
            draws 
        }
    }

    fn possible(&self, maximums:&Cubes) -> bool {
        self.draws.iter().all(|c| c.possible(maximums))
    }

    fn power(&self) -> u32 {
        let zeros = Cubes::zeros();
        self.draws.iter().fold(zeros, |l, r| l.max(&r)).prod()
    }
}