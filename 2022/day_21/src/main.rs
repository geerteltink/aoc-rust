#![allow(unused_imports)]

use aoc::*;

static DAY: &'static str = "21";

#[derive(Debug, Clone)]
struct Monkey {
    a: Option<String>,
    op: Option<String>,
    b: Option<String>,
    value: Option<i64>,
}

fn resolve_name(name: &str, monkeys: &HashMap<String, Monkey>) -> i64 {
    let monkey = monkeys.get(name).unwrap();
    
    if monkey.value.is_some() {
        return monkey.value.unwrap();
    }
    
    let a = resolve_name(monkey.a.as_ref().unwrap(), &monkeys);
    let b = resolve_name(monkey.b.as_ref().unwrap(), &monkeys);
    match monkey.op.as_ref().unwrap().clone().as_str() {
        "+" => return a + b,
        "-" => return a - b,
        "*" => return a * b,
        "/" => return a / b,
        _ => panic!("Unknown operator"),
    }
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
    let mut monkeys: HashMap<String, Monkey> = HashMap::new();
    
    for line in input.lines() {
        let (name, yell) = line.trim().split_once(":").unwrap();
        if let Ok(x) = yell.trim().parse::<i64>() {
            monkeys.insert(name.to_string(), Monkey {
                a: None,
                op: None,
                b: None,
                value: Some(x),
            });
            continue;
        }
        
        let (a, op, b) = yell.trim().split_whitespace().collect_tuple().unwrap();
        monkeys.insert(name.to_string(), Monkey {
            a: Some(a.to_string()),
            op: Some(op.to_string()),
            b: Some(b.to_string()),
            value: None,
        });
    }

    return resolve_name("root", &monkeys);
}

fn part_two(input: &String) -> isize {
    let mut monkeys: HashMap<String, Monkey> = HashMap::new();
    
    for line in input.lines() {
        let (name, yell) = line.trim().split_once(":").unwrap();
        if let Ok(x) = yell.trim().parse::<i64>() {
            monkeys.insert(name.to_string(), Monkey {
                a: None,
                op: None,
                b: None,
                value: Some(x),
            });
            continue;
        }
        
        let (a, op, b) = yell.trim().split_whitespace().collect_tuple().unwrap();
        monkeys.insert(name.to_string(), Monkey {
            a: Some(a.to_string()),
            op: Some(op.to_string()),
            b: Some(b.to_string()),
            value: None,
        });
    }
    
    let root = monkeys.get("root").unwrap();

    let left = resolve_name(root.a.as_ref().unwrap(), &monkeys);
    let right = resolve_name(root.b.as_ref().unwrap(), &monkeys);
    
    dbg!(left, right);
    
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_the_test_answer_for_part_one() {
        let input = load_input("./fixtures/input_test.txt");
        assert_eq!(152, part_one(&input));
    }

    #[test]
    fn it_returns_the_test_answer_for_part_two() {
        let input = load_input("./fixtures/input_test.txt");
        assert_eq!(301, part_two(&input));
    }
    
    #[test]
    fn it_returns_the_answer_for_part_one() {
        let input = load_input("./fixtures/input.txt");
        assert_eq!(83056452926300, part_one(&input));
    }
    
    #[ignore]
    #[test]
    fn it_returns_the_answer_for_part_two() {
        let input = load_input("./fixtures/input.txt");
        assert_eq!(0, part_two(&input));
    }
}
