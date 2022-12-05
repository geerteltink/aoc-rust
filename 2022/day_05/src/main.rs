static DAY: &'static str = "05";

use core::panic;
use regex::Regex;
use std::str::FromStr;

fn main() {
    let input = std::fs::read_to_string(format!("./2022/day_{DAY}/fixtures/input.txt")).unwrap();

    let result1 = part_one(&input);
    println!("Answer day {DAY} part one: {result1}");

    let result2 = part_two(&input);
    println!("Answer day {DAY} part two: {result2}");
}

fn part_one(input: &String) -> String {
    let (cargo, actions) = input.split_once("\n\n").unwrap();
    let mut stacks = load_crates(&cargo);

    for action in actions.lines() {
        move_crates(action, &mut stacks, true);
    }

    let answer = get_top_crates(&stacks);
    
    // stacks become borrowed in here, so we need print them after processing
    //for stack in stacks {
    //    println!("{:?}", stack);
    //}

    return answer;
}

fn part_two(input: &String) -> String {
    let (cargo, actions) = input.split_once("\n\n").unwrap();
    let mut stacks = load_crates(&cargo);

    for action in actions.lines() {
        move_crates(action, &mut stacks, false);
    }

    return get_top_crates(&stacks);
}

// Some crazy stuff happening in here, no wonder I could not get it working at the first try
fn load_crates(cargo: &str) -> Vec<Vec<char>> {
    // Collect cargo
    let mut cargo: Vec<_> = cargo.lines().collect();
    // Get number of stacks
    let num_stacks = cargo.pop().unwrap().split_ascii_whitespace().count();
    // Create a vector now that the number stacks is known
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); num_stacks];
    
    // Start constructing the stacks from bottom to top
    cargo
        .iter() // Iterate over lines
        .rev() // bottom up
        .map(|row| {
            row.chars() // convert ro to characters
                .skip(1) // skip first "[" character
                .step_by(4) // get every 4th character, including the first one
                .enumerate() // get a pair with the current index and value
                .filter(|(_, c)| !c.is_ascii_whitespace()) // filter out white space (empty stacks)
        })
        .flatten() // flatten structure
        .for_each(|(i, c)| stacks[i].push(c)); // push the value to its corresponding stack
        
    return stacks;
}

fn move_crates(action: &str, stacks: &mut Vec<Vec<char>>, single: bool) {
    // only handle move actions
    let re = Regex::new(r"move (?P<count>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();
    if re.is_match(action) == false {
        return;
    }

    let caps = re.captures(action).unwrap();

    let count: usize = FromStr::from_str(&caps["count"]).unwrap();
    let from: usize = FromStr::from_str(&caps["from"]).unwrap();
    let to: usize = FromStr::from_str(&caps["to"]).unwrap();

    if stacks[from - 1].len() < count {
        panic!("Not enough crates on the stack");
    }

    // Move crates one by one (step 1)
    let mut stack = vec![];
    for _ in 0..count {
        let label = stacks[from - 1].pop().unwrap();
        stack.push(label);
    }

    // In part 2 crates are moved in 1 go, so the reverse order is needed
    if single == false {
        stack.reverse();
    }

    stacks[to - 1].extend(stack);
}

fn get_top_crates(stacks: &Vec<Vec<char>>) -> String {
    stacks.iter().filter_map(|s| s.last()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_the_answer_for_part_one() {
        let input = std::fs::read_to_string("./fixtures/input_test.txt").unwrap();
        assert_eq!("CMZ", part_one(&input));
    }

    #[test]
    fn it_returns_the_answer_for_part_two() {
        let input = std::fs::read_to_string("./fixtures/input_test.txt").unwrap();
        assert_eq!("MCD", part_two(&input));
    }
}
