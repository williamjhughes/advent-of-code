use puzzle_one::puzzle_one;
use puzzle_two::puzzle_two;

mod puzzle_one;
mod puzzle_two;
mod shared;

pub const INPUT_DATA: &str = "inputs/data.txt";
pub const INPUT_TEST: &str = "inputs/test.txt";

fn main() {
    let puzzle_one_result = puzzle_one(INPUT_DATA);
    let puzzle_two_result = puzzle_two(INPUT_DATA);

    println!("Part 1: {puzzle_one_result}");
    println!("Part 2: {puzzle_two_result}");
}

#[cfg(test)]
mod tests {
    use super::{puzzle_one, puzzle_two, INPUT_TEST};

    #[test]
    fn it_solves_puzzle_one() {
        let result = puzzle_one(INPUT_TEST);
        assert_eq!(result, 11);
    }

    #[test]
    fn it_solves_puzzle_two() {
        let result = puzzle_two(INPUT_TEST);
        assert_eq!(result, 31);
    }
}
