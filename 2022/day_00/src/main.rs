#![allow(unused_imports)]

use aoc::*;

static DAY: &'static str = "00";

fn main() {
    let input = load_input(format!("./2022/day_{DAY}/fixtures/input.txt"));

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

fn part_one(_input: &String) -> isize {
    return 0;
}

fn part_two(_input: &String) -> isize {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_the_test_answer_for_part_one() {
        let input = load_input("./fixtures/input_test.txt");
        assert_eq!(0, part_one(&input));
    }

    #[test]
    fn it_returns_the_test_answer_for_part_two() {
        let input = load_input("./fixtures/input_test.txt");
        assert_eq!(0, part_two(&input));
    }
    
    #[ignore]
    #[test]
    fn it_returns_the_answer_for_part_one() {
        let input = load_input("./fixtures/input.txt");
        assert_eq!(0, part_one(&input));
    }
    
    #[ignore]
    #[test]
    fn it_returns_the_answer_for_part_two() {
        let input = load_input("./fixtures/input.txt");
        assert_eq!(0, part_two(&input));
    }
}
