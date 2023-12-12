#![allow(unused_imports)]

use aoc::*;

static DAY: &'static str = "06";

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
    let (line1, line2) = input.split_once("\n").unwrap();
    let times: Vec<usize> = line1
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();
    let distances: Vec<usize> = line2
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect();
    count_ways(&times, &distances)
}

fn part_two(input: &String) -> usize {
    let (line1, line2) = input.split_once("\n").unwrap();

    let (_, time) = line1.split_once(":").unwrap();
    let times: Vec<usize> = vec![time.trim().replace(' ', "").parse().unwrap()];

    let (_, distance) = line2.split_once(":").unwrap();
    let distances: Vec<usize> = vec![distance.trim().replace(' ', "").parse().unwrap()];

    count_ways(&times, &distances)
}

fn count_ways(times: &[usize], distances: &[usize]) -> usize {
    let mut total_ways = 1;
    for (t, d) in times.iter().zip(distances.iter()) {
        total_ways *= (1..*t).filter(|i| i * (*t - i) > *d).count() as usize;
    }
    total_ways
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_the_test_answer_for_part_one() {
        let input = load_input("./fixtures/input_test.txt");
        assert_eq!(288, part_one(&input));
    }

    #[test]
    fn it_returns_the_answer_for_part_one() {
        let input = load_input("./fixtures/input.txt");
        assert_eq!(1660968, part_one(&input));
    }

    #[test]
    fn it_returns_the_test_answer_for_part_two() {
        let input = load_input("./fixtures/input_test.txt");
        assert_eq!(71503, part_two(&input));
    }

    #[ignore]
    #[test]
    fn it_returns_the_answer_for_part_two() {
        let input = load_input("./fixtures/input.txt");
        assert_eq!(0, part_two(&input));
    }
}
