static DAY: &'static str = "05";

use core::panic;
use regex::Regex;
use std::str::FromStr;

fn main() {
    let content = std::fs::read_to_string(format!("./2022/day_{DAY}/fixtures/input.txt")).unwrap();

    let stacks = vec![
        String::from("GTRW"),
        String::from("GCHPMSVW"),
        String::from("CLTSGM"),
        String::from("JHDMWRF"),
        String::from("PQLHSWFJ"),
        String::from("PJDNFMS"),
        String::from("ZBDFGCSJ"),
        String::from("RTB"),
        String::from("HNWLC"),
    ];

    let result1 = part_one(&content, &stacks);
    println!("Answer day {DAY} part one: {result1}");

    let result2 = part_two(&content, &stacks);
    println!("Answer day {DAY} part two: {result2}");
}

fn part_one(content: &String, stacks: &Vec<String>) -> String {
    let mut stacks = stacks.clone();

    for action in content.lines() {
        move_crates(action, &mut stacks, true)
    }

    return get_top_labels(&stacks);
}

fn part_two(content: &String, stacks: &Vec<String>) -> String {
    let mut stacks = stacks.clone();

    for action in content.lines() {
        move_crates(action, &mut stacks, false)
    }

    return get_top_labels(&stacks);
}

fn move_crates(action: &str, stacks: &mut Vec<String>, single: bool) {
    let re = Regex::new(r"move (?P<count>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();

    // only handle move actions
    if re.is_match(action) == false {
        return;
    }

    let caps = re.captures(action).unwrap();

    let count: usize = FromStr::from_str(&caps[1]).unwrap();
    let from: usize = FromStr::from_str(&caps[2]).unwrap();
    let to: usize = FromStr::from_str(&caps[3]).unwrap();

    if stacks[from - 1].len() < count {
        panic!("Not enough crates on the stack");
    }

    let mut stack = vec![];
    for _ in 0..count {
        let label = stacks[from - 1].pop().unwrap();
        stack.push(label);
    }

    // part 2
    if single == false {
        stack.reverse();
    }

    stacks[to - 1].extend(stack);
}

fn get_top_labels(stacks: &Vec<String>) -> String {
    let mut answer = String::from("");
    for stack in stacks {
        answer.push(stack.chars().last().unwrap());
    }

    return answer;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_the_answer_for_part_one() {
        let stacks = vec![String::from("ZN"), String::from("MCD"), String::from("P")];

        let content = std::fs::read_to_string("./fixtures/input_test.txt").unwrap();
        assert_eq!("CMZ", part_one(&content, &stacks));
    }

    #[test]
    fn it_returns_the_answer_for_part_two() {
        let stacks = vec![String::from("ZN"), String::from("MCD"), String::from("P")];

        let content = std::fs::read_to_string("./fixtures/input_test.txt").unwrap();
        assert_eq!("MCD", part_two(&content, &stacks));
    }
}
