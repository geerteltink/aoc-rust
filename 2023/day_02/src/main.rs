#![allow(unused_imports)]

use aoc::*;
use std::convert::identity;

static DAY: &'static str = "02";

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

fn part_one(input: &String) -> usize {
    let games: Vec<Game> = input.lines().map(|line| Game::from_string(line)).collect();

    games
        .iter()
        .enumerate()
        .filter_map(|(id, game)| {
            for set in &game.sets {
                if set.0 > 12 || set.1 > 13 || set.2 > 14 {
                    return None;
                }
            }
            Some(id + 1)
        })
        .sum()
}

fn part_two(input: &String) -> u32 {
    let games: Vec<Game> = input.lines().map(|line| Game::from_string(line)).collect();

    games
        .iter()
        .map(|game| {
            let mut min_set = game.sets[0];
            for set in &game.sets {
                min_set.0 = min_set.0.max(set.0);
                min_set.1 = min_set.1.max(set.1);
                min_set.2 = min_set.2.max(set.2);
            }
            min_set.0 as u32 * min_set.1 as u32 * min_set.2 as u32
        })
        .sum()
}

// Nice extraction, found on redit adventofcode solutions day 2
struct Game {
    sets: Vec<Set>,
}

impl Game {
    fn from_string(line: &str) -> Game {
        let mut sets = Vec::new();
        let suffix = line.split_once(": ").unwrap().1;
        for part in suffix.split("; ") {
            sets.push(Set::from_string(part));
        }
        Game { sets }
    }
}

#[derive(Clone, Copy)]
struct Set(u8, u8, u8);
impl Set {
    fn from_string(str: &str) -> Set {
        let mut out = Set(0, 0, 0);
        for part in str.split(", ") {
            let (num, color) = part.split_once(" ").unwrap();
            match color {
                "red" => out.0 = num.parse().unwrap(),
                "green" => out.1 = num.parse().unwrap(),
                "blue" => out.2 = num.parse().unwrap(),
                _ => unreachable!("Not a color! {part}"),
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_the_test_answer_for_part_one() {
        let input = load_input("./fixtures/input_test.txt");
        assert_eq!(8, part_one(&input));
    }

    #[test]
    fn it_returns_the_answer_for_part_one() {
        let input = load_input("./fixtures/input.txt");
        assert_eq!(2600, part_one(&input));
    }

    #[test]
    fn it_returns_the_test_answer_for_part_two() {
        let input = load_input("./fixtures/input_test.txt");
        assert_eq!(2286, part_two(&input));
    }

    #[test]
    fn it_returns_the_answer_for_part_two() {
        let input = load_input("./fixtures/input.txt");
        assert_eq!(86036, part_two(&input));
    }
}
