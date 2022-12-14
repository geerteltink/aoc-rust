use aoc::*;
use serde_json::Value;
use std::cmp::Ordering;

// https://betterprogramming.pub/how-to-work-with-json-in-rust-35ddc964009e

static DAY: &'static str = "13";

fn compare(a: &Value, b: &Value) -> Ordering {
    match (a, b) {
        // If both values are integers, the lower integer should come first.
        // If the left integer is lower than the right integer, the inputs are in the right order.
        // If the left integer is higher than the right integer, the inputs are not in the right order.
        // Otherwise, the inputs are the same integer; continue checking the next part of the input.
        (Value::Number(a), Value::Number(b)) => a.as_u64().unwrap().cmp(&b.as_u64().unwrap()),

        // If both values are lists, compare the first value of each list, then the second value, and so on.
        // If the left list runs out of items first, the inputs are in the right order.
        // If the right list runs out of items first, the inputs are not in the right order.
        // If the lists are the same length and no comparison makes a decision about the order,
        // continue checking the next part of the input.
        (Value::Array(a), Value::Array(b)) => {
            for (a, b) in a.iter().zip(b.iter()) {
                let result = compare(a, b);
                if !result.is_eq() {
                    return result;
                }
            }

            return a.len().cmp(&b.len());
        }

        // If exactly one value is an integer, convert the integer to a list
        // which contains that integer as its only value, then retry the comparison.
        // For example, if comparing [0,0,0] and 2, convert the right value to [2]
        // (a list containing 2); the result is then found by instead comparing [0,0,0] and [2].
        (Value::Array(_), Value::Number(_)) => compare(a, &Value::Array(vec![b.clone()])),
        (Value::Number(_), Value::Array(_)) => compare(&Value::Array(vec![a.clone()]), b),
        _ => unreachable!(),
    }
}

fn main() {
    let input = load_input(format!("./2022/day_{DAY}/fixtures/input.txt"));

    let result1 = part_one(&input);
    println!("Answer day {DAY} part one: {result1}");

    let result2 = part_two(&input);
    println!("Answer day {DAY} part one: {result2}");
}

fn part_one(input: &String) -> usize {
    let mut count = 0;

    for (i, pairs) in input.split("\n\n").enumerate() {
        let (left, right) = pairs.split_once('\n').unwrap();

        // Parse strings as json
        let left: Value = serde_json::from_str(left.trim()).unwrap();
        let right: Value = serde_json::from_str(right.trim()).unwrap();

        // Get the sum of the indices of the pairs that are already in the right order
        if compare(&left, &right).is_lt() {
            count += i + 1;
        }
    }

    return count;
}

fn part_two(input: &String) -> usize {
    let mut packets = vec![];

    for (_i, pairs) in input.split("\n\n").enumerate() {
        let (left, right) = pairs.split_once('\n').unwrap();

        // Parse strings as json
        let left: Value = serde_json::from_str(left.trim()).unwrap();
        let right: Value = serde_json::from_str(right.trim()).unwrap();

        packets.push(left);
        packets.push(right);
    }

    let divider_packet_1 = Value::Array(vec![Value::Array(vec![Value::Number((2).into())])]);
    let divider_packet_2 = Value::Array(vec![Value::Array(vec![Value::Number((6).into())])]);

    // Include two additional divider packets
    packets.push(divider_packet_1.clone());
    packets.push(divider_packet_2.clone());

    // Organize all of the packets into the correct order
    // Using the same rules as before
    packets.sort_by(compare);

    // Locate the divider packet positions
    let divider1 = packets.iter().position(|x| x == &divider_packet_1).unwrap() + 1;
    let divider2 = packets.iter().position(|x| x == &divider_packet_2).unwrap() + 1;

    debug_println!("Divider packet positions: {divider1}, {divider2}");

    return divider1 * divider2;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_the_answer_for_part_one() {
        let input = load_input("./fixtures/input_test.txt");
        assert_eq!(13, part_one(&input));
    }

    #[test]
    fn it_returns_the_answer_for_part_two() {
        let input = load_input("./fixtures/input_test.txt");
        assert_eq!(140, part_two(&input));
    }
}
