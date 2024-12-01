// Problem name Historian Hysteria
// <https://adventofcode.com/2024/day1>

#[allow(dead_code)]
pub fn day01_a(input: String) -> usize {
    return 0;
}

pub fn day01_b(input: String) -> usize {
    return 0;
}

#[cfg(test)]
mod tests_day01 {
    use std::fs;

    use super::*;

    #[test]
    fn test_day01_a() {
        let input_1: String =
            fs::read_to_string("src/day01/input_1a.txt").expect("Error reading input_1.txt");

        let test_1: usize = 0;

        assert_eq!(day01_a(input_1), test_1);
    }

    /// this test is for the second input file update the answer after submission
    #[test]
    fn test_day01_a_long() {
        let input_2: String =
            fs::read_to_string("src/day01/input_2a.txt").expect("Error reading input_2.txt");

        let test_2: usize = 0;

        assert_eq!(day01_a(input_2), test_2);
    }

    // --- Second part of the day
    #[test]
    fn test_day01_b() {
        let input_1: String =
            fs::read_to_string("src/day01/input_1b.txt").expect("Error reading input_1b.txt");

        let test_1: usize = 0;

        assert_eq!(day01_b(input_1), test_1);
    }

    /// this test is for the second input file update the answer after submission
    #[test]
    fn test_day01_b_long() {
        let input_2: String =
            fs::read_to_string("src/day01/input_2b.txt").expect("Error reading input_2b.txt");

        let test_2: usize = 0;

        assert_eq!(day01_a(input_2), test_2);
    }
}
