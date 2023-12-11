#![allow(unused_imports)]

use aoc::*;

static DAY: &'static str = "01";

fn main() {
    let input = load_input(format!("./2023/day_{DAY}/fixtures/input.txt"));

    let start = Instant::now();
    let result1 = part_one(&input);
    let end = Instant::now();
    println!("Elapsed time: {:?}", end - start);
    println!("Answer day {DAY} part one: {result1} in {:?}", end - start);

    let start = Instant::now();
    let result2 = part_two(&input);
    let end = Instant::now();
    println!("Answer day {DAY} part two: {result2} in {:?}", end - start);
}

fn part_one(input: &String) -> u32 {
    input.lines().map(get_calibration_value).sum()
}

fn part_two(input: &String) -> u32 {
    input.lines().map(get_parsed_calibration_value).sum()
}

fn get_calibration_value(line: &str) -> u32 {
    let first = line.chars().find_map(|c| c.to_digit(10));
    let last = line.chars().rev().find_map(|c| c.to_digit(10));
    10 * first.unwrap() + last.unwrap()
}

fn get_parsed_calibration_value(line: &str) -> u32 {
    let line = line
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");

    get_calibration_value(&line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_the_test_answer_for_part_one() {
        let input = load_input("./fixtures/input_test.txt");
        assert_eq!(142, part_one(&input));
    }

    #[test]
    fn it_returns_the_answer_for_part_one() {
        let input = load_input("./fixtures/input.txt");
        assert_eq!(57346, part_one(&input));
    }

    #[test]
    fn it_returns_the_test_answer_for_part_two() {
        let input = load_input("./fixtures/input_test2.txt");
        assert_eq!(281, part_two(&input));
    }

    #[test]
    fn it_returns_the_answer_for_part_two() {
        let input = load_input("./fixtures/input.txt");
        assert_eq!(57345, part_two(&input));
    }
}
