use std::collections::HashMap;

use crate::shared;

pub fn puzzle_two(input: &str) -> i32 {
    match shared::parse_locations_vectors(input) {
        Ok((locations_one, locations_two)) => {
            let mut locations_count = HashMap::new();

            for &value in &locations_two {
                *locations_count.entry(value).or_insert(0) += 1;
            }

            let similarity_score: i32 = locations_one
                .iter()
                .filter_map(|value| locations_count.get(value).map(|&count| value * count))
                .sum();

            return similarity_score;
        }
        Err(e) => println!("Failed to Parse Locations: {}", e),
    }

    -1
}
