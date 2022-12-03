static DAY: &'static str = "00";

fn main() {
    let content = std::fs::read_to_string(format!("./2022/day_{DAY}/fixtures/input.txt")).unwrap();

    let result1 = part_one(&content);
    println!("Answer day {DAY} part one: {result1}");

    let result2 = part_two(&content);
    println!("Answer day {DAY} part one: {result2}");
}

fn part_one(_content: &String) -> i32 {
    return 0;
}

fn part_two(_content: &String) -> i32 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn it_returns_the_answer_for_part_one() {
        let content = std::fs::read_to_string("./fixtures/input_test.txt").unwrap();
        assert_eq!(0, part_one(&content));
    }

    #[ignore]
    #[test]
    fn it_returns_the_answer_for_part_two() {
        let content = std::fs::read_to_string("./fixtures/input_test.txt").unwrap();
        assert_eq!(0, part_two(&content));
    }
}
