#![allow(unused_imports)]

use aoc::*;

static DAY: &'static str = "09";

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

fn part_one(input: &String) -> i32 {
    let mut sum: i32 = 0;

    for line in input.lines() {
        let mut numbers: Vec<i32> = line.split(" ").map(|s| s.parse().unwrap()).collect();
        let mut current: i32 = 0;

        while !numbers.iter().all(|i| *i == 0) {
            current += numbers.last().unwrap();

            let mut new_numbers = vec![0; numbers.len() - 1];
            for i in 0..numbers.len() - 1 {
                new_numbers[i] = numbers[i + 1] - numbers[i];
            }
            numbers = new_numbers;
        }
        sum += current;
    }

    return sum;
}

fn part_two(input: &String) -> i32 {
    let mut sum: i32 = 0;

    for line in input.lines() {
        let mut numbers: Vec<i32> = line.split(" ").map(|s| s.parse().unwrap()).collect();
        numbers = numbers.into_iter().rev().collect();
        let mut current: i32 = 0;

        while !numbers.iter().all(|i| *i == 0) {
            current += numbers.last().unwrap();

            let mut new_numbers = vec![0; numbers.len() - 1];
            for i in 0..numbers.len() - 1 {
                new_numbers[i] = numbers[i + 1] - numbers[i];
            }
            numbers = new_numbers;
        }
        sum += current;
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_the_test_answer_for_part_one() {
        let input = load_input("./fixtures/input_test.txt");
        assert_eq!(114, part_one(&input));
    }

    #[test]
    fn it_returns_the_answer_for_part_one() {
        let input = load_input("./fixtures/input.txt");
        assert_eq!(2008960228, part_one(&input));
    }

    #[test]
    fn it_returns_the_test_answer_for_part_two() {
        let input = load_input("./fixtures/input_test.txt");
        assert_eq!(2, part_two(&input));
    }

    #[test]
    fn it_returns_the_answer_for_part_two() {
        let input = load_input("./fixtures/input.txt");
        assert_eq!(1097, part_two(&input));
    }
}
