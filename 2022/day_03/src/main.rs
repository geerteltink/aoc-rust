static DAY: &'static str = "03";

fn main() {
    let content = std::fs::read_to_string(format!("./2022/day_{DAY}/fixtures/input.txt")).unwrap();

    let result1 = part_one(&content);
    println!("Result day {DAY} part one: {result1}");

    let result2 = part_two(&content);
    println!("Result day {DAY} part one: {result2}");
}

fn part_one(_content: &String) -> i32 {
    return 1;
}

fn part_two(_content: &String) -> i32 {
    return 2;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_the_answer_for_part_one() {
        let content = std::fs::read_to_string("./fixtures/input_test.txt").unwrap();
        assert_eq!(1, part_one(&content));
    }

    #[test]
    fn it_returns_the_answer_for_part_two() {
        let content = std::fs::read_to_string("./fixtures/input_test.txt").unwrap();
        assert_eq!(2, part_two(&content));
    }
}
