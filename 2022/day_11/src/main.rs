use std::mem::take;
use itertools::Itertools;
use num::Integer;

static DAY: &'static str = "11";

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Op {
    Add(i64),
    Mul(i64),
    Sqr,
}

impl Op {
    fn new(op: &[&str]) -> Self {
        match op[1] {
            "*" => if op[2] == "old" { Op::Sqr } else { Op::Mul(op[2].parse::<i64>().unwrap()) },
            "+" => Op::Add(op[2].parse::<i64>().unwrap()),
            _ => panic!("Unknown operation"),
        }
    }
    
    fn apply(&self, v: i64) -> i64 {
        match self {
            Op::Sqr => v * v,
            Op::Mul(v2) => v * v2,
            Op::Add(v2) => v + v2,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Monkey {
    items: Vec<i64>,
    op: Op,
    divisible_test: i64,
    throw_to_monkey_if_true: usize,
    throw_to_monkey_if_false: usize,
}

fn main() {
    let input = std::fs::read_to_string(format!("./2022/day_{DAY}/fixtures/input.txt")).unwrap();

    let result1 = part_one(&input);
    println!("Answer day {DAY} part one: {result1}");

    let result2 = part_two(&input);
    println!("Answer day {DAY} part one: {result2}");
}

fn part_one(input: &String) -> i64 {
    fn split_line(s: &str) -> impl Iterator<Item = &str> {
        s.trim().split(&[' ', ',']).filter(|s| !s.is_empty())
    }
    
    let mut monkeys = input
        .split("\n\n")
        .map(|x| {
            let lines = x.lines().collect_vec();
            
            // This needs improvement, probably a way to strip out numbers
            Monkey {
                items: split_line(lines[1]).skip(2).map(|v| v.parse::<i64>().unwrap()).collect_vec(),
                op: Op::new(&split_line(lines[2]).skip(3).collect::<Vec<&str>>()),
                divisible_test: split_line(lines[3]).nth(3).unwrap().parse::<i64>().unwrap(),
                throw_to_monkey_if_true: split_line(lines[4]).nth(5).unwrap().parse().unwrap(),
                throw_to_monkey_if_false: split_line(lines[5]).nth(5).unwrap().parse().unwrap(),
            }
        })
        .collect_vec();
    
    let mut inspections = vec![0i64; monkeys.len()];
    
    // play 20 rounds
    for round in 0..20 {
        // every monkey inspects all its items
        for i in 0..monkeys.len() {
            for item in take(&mut monkeys[i].items) {
                let mut item = item;
                
                // Monkey inspects an item with a worry level               
                inspections[i] += 1;
                
                // Calculate new worry level
                match monkeys[i].op.clone() {
                    Op::Add(x) => item += x,
                    Op::Mul(x) => item *= x,
                    Op::Sqr => item *= item,
                }
                
                // Monkey gets bored with item, worry level is divided by 3
                item /= 3;
                
                // Is current worry level divisible
                let dest = if item % monkeys[i].divisible_test == 0 {
                    monkeys[i].throw_to_monkey_if_true
                } else {
                    monkeys[i].throw_to_monkey_if_false
                };

                // Throw item to other monkey
                monkeys[dest].items.push(item);
            }
        }
        
        //println!("{round} -> {:?}", monkeys);
    }

    inspections.sort();
    inspections.reverse();

    return inspections[0] * inspections[1];
}

fn part_two(input: &String) -> i64 {
    fn split_line(s: &str) -> impl Iterator<Item = &str> {
        s.trim().split(&[' ', ',']).filter(|s| !s.is_empty())
    }
    
    let mut monkeys = input
        .split("\n\n")
        .map(|x| {
            let lines = x.lines().collect_vec();
            
            // This needs improvement, probably a way to strip out numbers
            Monkey {
                items: split_line(lines[1]).skip(2).map(|v| v.parse::<i64>().unwrap()).collect_vec(),
                op: Op::new(&split_line(lines[2]).skip(3).collect::<Vec<&str>>()),
                divisible_test: split_line(lines[3]).nth(3).unwrap().parse::<i64>().unwrap(),
                throw_to_monkey_if_true: split_line(lines[4]).nth(5).unwrap().parse().unwrap(),
                throw_to_monkey_if_false: split_line(lines[5]).nth(5).unwrap().parse().unwrap(),
            }
        })
        .collect_vec();
    
    let mut inspections = vec![0i64; monkeys.len()];
       
    // play 20 rounds
    for _ in 0..10000 {
        // keep worry levels manageable with the lowest common multiple
        let mut modulo = 1i64;
        for divis in monkeys.iter().map(|x| x.divisible_test) {
            modulo = modulo.lcm(&divis);
        }
        
        // every monkey inspects all its items
        for i in 0..monkeys.len() {
            for item in take(&mut monkeys[i].items) {
                let mut item = item;
                
                // Monkey inspects an item with a worry level               
                inspections[i] += 1;
                
                // Calculate new worry level
                match monkeys[i].op.clone() {
                    Op::Add(x) => item += x,
                    Op::Mul(x) => item *= x,
                    Op::Sqr => item *= item,
                }
                
                // Monkey gets bored with item, worry level is divided by 3
                //item /= 3;
                
                // Is current worry level divisible
                let dest = if item % monkeys[i].divisible_test == 0 {
                    monkeys[i].throw_to_monkey_if_true
                } else {
                    monkeys[i].throw_to_monkey_if_false
                };

                // Throw item to other monkey
                monkeys[dest].items.push(item % modulo);
            }
        }
        
        //println!("{round} -> {:?}", monkeys);
    }

    inspections.sort();
    inspections.reverse();

    return inspections[0] * inspections[1];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_the_answer_for_part_one() {
        let input = std::fs::read_to_string("./fixtures/input_test.txt").unwrap();
        assert_eq!(10605, part_one(&input));
    }

    #[test]
    fn it_returns_the_answer_for_part_two() {
        let input = std::fs::read_to_string("./fixtures/input_test.txt").unwrap();
        assert_eq!(2713310158, part_two(&input));
    }
}
