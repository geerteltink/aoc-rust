#![allow(unused_imports)]

use aoc::*;

static DAY: &'static str = "20";

fn decrypt(input: &String, repetitions: i32, decryption_key: i64) -> i64 {
    // apply the decryption key
    let mut numbers = input
        .lines()
        .enumerate()
        .map(|(i, x)| (i, x.int() * decryption_key))
        .collect_vec();

    let orig = numbers.clone();

    // mix the list of numbers ten times
    for _ in 0..repetitions {
        for number in orig.clone() {
            let index = numbers.iter().position(|&x| x == number).unwrap();
            numbers.remove(index);
            
            let new_index = (index.int() + number.1).rem_euclid(numbers.len().int());
            numbers.insert(new_index.uint(), number);
        }
    }

    // skip zero
    let zero_index = numbers.iter().position(|&x| x.1 == 0).unwrap();

    // wrapping around the list
    let expanded = numbers
        .iter()
        .cycle()
        .skip(zero_index)
        .map(|x| x.1)
        .take(3001)
        .collect_vec();

    // return grove coordinates
    return expanded[1000] + expanded[2000] + expanded[3000];
}

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

fn part_one(input: &String) -> i64 {
    decrypt(input, 1, 1)
}

fn part_two(input: &String) -> i64 {
    decrypt(input, 10, 811589153)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_the_test_answer_for_part_one() {
        let input = load_input("./fixtures/input_test.txt");
        assert_eq!(3, part_one(&input));
    }

    #[test]
    fn it_returns_the_test_answer_for_part_two() {
        let input = load_input("./fixtures/input_test.txt");
        assert_eq!(1623178306, part_two(&input));
    }

    #[test]
    fn it_returns_the_answer_for_part_one() {
        let input = load_input("./fixtures/input.txt");
        assert_eq!(7278, part_one(&input));
    }

    #[test]
    fn it_returns_the_answer_for_part_two() {
        let input = load_input("./fixtures/input.txt");
        assert_eq!(14375678667089, part_two(&input));
    }
}
