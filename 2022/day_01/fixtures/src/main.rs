fn main() {
    let content = std::fs::read_to_string("./2022/day_01/fixtures/input.txt").unwrap();

    let result1 = part_one(&content);
    println!("Result part one: {result1}");

    let result2 = part_two(&content);
    println!("Result part one: {result2}");
}

fn part_one(content: &String) -> i32 {
    return i32::try_from(content.len()).unwrap();
}

fn part_two(content: &String) -> i32 {
    return i32::try_from(content.len()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_part_one_result() {
        let content = std::fs::read_to_string("./fixtures/input_test.txt").unwrap();
        assert_eq!(1, part_one(&content));
    }

    #[test]
    fn check_part_two_result() {
        let content = std::fs::read_to_string("./fixtures/input_test.txt").unwrap();
        assert_eq!(2, part_two(&content));
    }
}
