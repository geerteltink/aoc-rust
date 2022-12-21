use aoc::*;
use parse_display::{Display, FromStr};
use xxcalc::calculator::Calculator;
use xxcalc::linear_solver::LinearSolver;

static DAY: &'static str = "21";

// parse_display is a really nice way to import data
#[derive(Display, FromStr, Debug)]
enum Operation {
    #[display("{0}")]
    Number(usize),
    #[display("{0} + {1}")]
    Add(String, String),
    #[display("{0} - {1}")]
    Subtract(String, String),
    #[display("{0} * {1}")]
    Multiply(String, String),
    #[display("{0} / {1}")]
    Divide(String, String),
}

#[derive(Display, FromStr, Debug)]
#[display("{name}: {operation}")]
struct Monkey {
    name: String,
    operation: Operation,
}

fn resolve_name(monkeys: &HashMap<String, Operation>, monkey: &str) -> usize {
    match &monkeys[monkey] {
        Operation::Number(n) => *n,
        Operation::Add(a, b) => resolve_name(monkeys, a) + resolve_name(monkeys, b),
        Operation::Subtract(a, b) => resolve_name(monkeys, a) - resolve_name(monkeys, b),
        Operation::Multiply(a, b) => resolve_name(monkeys, a) * resolve_name(monkeys, b),
        Operation::Divide(a, b) => resolve_name(monkeys, a) / resolve_name(monkeys, b),
    }
}

fn build_expression(monkeys: &HashMap<String, Operation>, monkey_name: &str) -> String {
    if monkey_name == "humn" {
        return "x".to_string();
    }
    let monkey = &monkeys[monkey_name];
    match monkey {
        Operation::Number(n) => n.to_string(),
        Operation::Add(a, b) => format!(
            "({} + {})",
            build_expression(monkeys, a),
            build_expression(monkeys, b)
        ),
        Operation::Subtract(a, b) => format!(
            "({} - {})",
            build_expression(monkeys, a),
            build_expression(monkeys, b)
        ),
        Operation::Multiply(a, b) => format!(
            "({} * {})",
            build_expression(monkeys, a),
            build_expression(monkeys, b)
        ),
        Operation::Divide(a, b) => format!(
            "({} / {})",
            build_expression(monkeys, a),
            build_expression(monkeys, b)
        ),
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

fn part_one(input: &String) -> usize {
    let monkeys = input
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .map(|m: Monkey| (m.name, m.operation))
        .collect::<HashMap<_, _>>();

    return resolve_name(&monkeys, "root");
}

// Borrowed solution part two from https://github.com/MrRobb/advent-of-code-2022/blob/main/src/day21.rs
// Just because it is so much cooler than what I had
//
// @see https://docs.rs/xxcalc/latest/xxcalc/linear_solver/index.html
//
fn part_two(input: &String) -> usize {
    let monkeys = input
        .lines()
        .map(str::parse)
        .map(Result::unwrap)
        .map(|m: Monkey| (m.name, m.operation))
        .collect::<HashMap<_, _>>();

    let equation = match &monkeys["root"] {
        Operation::Add(a, b)
        | Operation::Subtract(a, b)
        | Operation::Multiply(a, b)
        | Operation::Divide(a, b) => {
            format!(
                "{} = {}",
                build_expression(&monkeys, a),
                resolve_name(&monkeys, b)
            )
        }
        Operation::Number(_) => unreachable!(),
    };

    return LinearSolver
        .process(&equation)
        .unwrap()
        .as_f64()
        .unwrap()
        .round() as usize;
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

    #[test]
    fn it_returns_the_answer_for_part_two() {
        let input = load_input("./fixtures/input.txt");
        assert_eq!(3469704905529, part_two(&input));
    }
}
