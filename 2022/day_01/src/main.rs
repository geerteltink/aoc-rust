static DAY: &'static str = "01";

fn main() {
    let content = std::fs::read_to_string(format!("./2022/day_{DAY}/fixtures/input.txt")).unwrap();

    let result1 = part_one(&content);
    println!("Result day {DAY} part one: {result1}");

    let result2 = part_two(&content);
    println!("Result day {DAY} part one: {result2}");
}

fn part_one(content: &String) -> u32 {
    let elfs = calculate_calories_per_elf(content);
    let answer = elfs.iter().max().unwrap();

    return *answer;
}

fn part_two(content: &String) -> u32 {
    let mut elfs = calculate_calories_per_elf(content);
    elfs.sort();

    let answer: u32 = elfs.iter().rev().take(3).sum();
    return answer;
}

fn calculate_calories_per_elf(content: &String) -> Vec<u32> {
    let mut elfs: Vec<u32> = Vec::new();

    let mut total_calories: u32 = 0;
    for line in content.lines() {
        if line == "" {
            elfs.push(total_calories);
            total_calories = 0;
        } else {
            let calories: u32 = line.trim().parse().unwrap();
            total_calories += calories;
        }
    }
    elfs.push(total_calories);

    return elfs;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_the_answer_for_part_one() {
        let content = std::fs::read_to_string("./fixtures/input_test.txt").unwrap();
        assert_eq!(24000, part_one(&content));
    }

    #[test]
    fn it_returns_the_answer_for_part_two() {
        let content = std::fs::read_to_string("./fixtures/input_test.txt").unwrap();
        assert_eq!(45000, part_two(&content));
    }

    #[test]
    fn it_calculates_calories_per_elf() {
        let content = std::fs::read_to_string("./fixtures/input_test.txt").unwrap();
        let expected: Vec<u32> = vec![6000, 4000, 11000, 24000, 10000];

        assert_eq!(expected, calculate_calories_per_elf(&content));
    }
}
