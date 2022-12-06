static DAY: &'static str = "07";

fn main() {
    let input = std::fs::read_to_string(format!("./2022/day_{DAY}/fixtures/input.txt")).unwrap();

    let result1 = part_one(&input);
    println!("Answer day {DAY} part one: {result1}");

    let result2 = part_two(&input);
    println!("Answer day {DAY} part one: {result2}");
}

fn part_one(_input: &String) -> i32 {
    return 0;
}

fn part_two(_input: &String) -> i32 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn it_returns_the_answer_for_part_one() {
        let input = std::fs::read_to_string("./fixtures/input_test.txt").unwrap();
        assert_eq!(0, part_one(&input));
    }

    #[ignore]
    #[test]
    fn it_returns_the_answer_for_part_two() {
        let input = std::fs::read_to_string("./fixtures/input_test.txt").unwrap();
        assert_eq!(0, part_two(&input));
    }
}
