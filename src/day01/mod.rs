// Problem name Historian Hysteria
// <https://adventofcode.com/2024/day1>

use std::collections::binary_heap::BinaryHeap;

#[allow(dead_code)]
fn day01_first(input: String) -> isize {
    let mut total_difference = 0;

    let mut first: BinaryHeap<i32> = BinaryHeap::new();
    let mut second: BinaryHeap<i32> = BinaryHeap::new();

    input.lines().for_each(|line| {
        line.split_whitespace()
            .take(2)
            .enumerate()
            .for_each(|(idx, num)| {
                if let Ok(parsed_number) = num.parse::<i32>() {
                    if idx == 0 {
                        first.push(-parsed_number);
                    } else {
                        second.push(-parsed_number);
                    }
                }
            });
    });

    while let (Some(num1), Some(num2)) = (first.pop(), second.pop()) {
        // inverting the signs again cuz minheap
        total_difference += (-num1 + num2).abs();
    }

    // crazy fuckery
    return total_difference.try_into().unwrap();
}

#[allow(dead_code)]
fn day01_second(input: String) -> isize {
    let mut total_sum = 0;

    let mut first: BinaryHeap<i32> = BinaryHeap::new();
    let mut second: BinaryHeap<i32> = BinaryHeap::new();

    input.lines().for_each(|line| {
        line.split_whitespace()
            .take(2)
            .enumerate()
            .for_each(|(idx, num)| {
                if let Ok(parsed_number) = num.parse::<i32>() {
                    if idx == 0 {
                        first.push(-parsed_number);
                    } else {
                        second.push(-parsed_number);
                    }
                }
            });
    });

    return total_sum;
}

#[cfg(test)]
mod tests_day01 {
    use std::fs;

    use super::*;

    #[test]
    fn test_day01_a() {
        let input_1: String =
            fs::read_to_string("src/day01/input_1a.txt").expect("Error reading input_1.txt");

        let test_1: isize = 11;

        assert_eq!(day01_first(input_1), test_1);
    }

    /// this test is for the second input file update the answer after submission
    #[test]
    fn test_day01_a_long() {
        let input_2: String =
            fs::read_to_string("src/day01/input_2a.txt").expect("Error reading input_2.txt");

        let test_2: isize = -1;

        assert_eq!(day01_first(input_2), test_2);
    }

    // --- Second part of the day
    #[test]
    fn test_day01_b() {
        let input_1: String =
            fs::read_to_string("src/day01/input_1b.txt").expect("Error reading input_1b.txt");

        let test_1: isize = -1;

        assert_eq!(day01_second(input_1), test_1);
    }

    /// this test is for the second input file update the answer after submission
    #[test]
    fn test_day01_b_long() {
        let input_2: String =
            fs::read_to_string("src/day01/input_2b.txt").expect("Error reading input_2b.txt");

        let test_2: isize = -1;

        assert_eq!(day01_second(input_2), test_2);
    }
}
