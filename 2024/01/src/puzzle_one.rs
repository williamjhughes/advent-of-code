use crate::shared;

pub fn puzzle_one(input: &str) -> i32 {
    match shared::parse_locations_vectors(input) {
        Ok((locations_one, locations_two)) => {
            let total_distance: i32 = locations_one
                .iter()
                .zip(locations_two)
                .map(|(one, two)| (one - two).abs())
                .sum();

            return total_distance;
        }
        Err(e) => println!("Failed to Parse Locations: {}", e),
    }

    -1
}
