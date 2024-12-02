use std::io::{self, BufRead};

pub fn parse_locations_vectors(input: &str) -> io::Result<(Vec<i32>, Vec<i32>)> {
    let mut locations_one = Vec::new();
    let mut locations_two = Vec::new();

    let file = std::fs::File::open(input)?;
    let buff = std::io::BufReader::new(file);

    for line in buff.lines() {
        if let Ok(line) = line {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .filter_map(|s| s.parse::<i32>().ok())
                .collect();

            if numbers.len() == 2 {
                locations_one.push(numbers[0]);
                locations_two.push(numbers[1]);
            }
        }
    }

    locations_one.sort();
    locations_two.sort();

    Ok((locations_one, locations_two))
}
