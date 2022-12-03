static DAY: &'static str = "04";

fn main() {
    let content = std::fs::read_to_string(format!("./2022/day_{DAY}/fixtures/input.txt")).unwrap();

    let result1 = part_one(&content);
    println!("Answer day {DAY} part one: {result1}");

    let result2 = part_two(&content);
    println!("Answer day {DAY} part one: {result2}");
}

fn part_one(content: &String) -> i32 {
    let mut full_intersections: i32 = 0;
    for line in content.lines() {
        let pairs = get_pairs(line);
        if has_full_intersection(pairs) {
            full_intersections += 1;
        }
    }

    return full_intersections;
}

fn part_two(content: &String) -> i32 {
    let mut full_intersections: i32 = 0;
    for line in content.lines() {
        let pairs = get_pairs(line);
        if has_any_intersection(pairs) {
            full_intersections += 1;
        }
    }

    return full_intersections;
}

fn get_pairs(sections: &str) -> (Vec<i32>, Vec<i32>) {
    let (pair_a, pair_b) = sections.trim().split_once(",").unwrap();

    let (pair_a_min, pair_a_max) = pair_a.trim().split_once("-").unwrap();
    let pair_a_min = pair_a_min.parse::<i32>().unwrap();
    let pair_a_max = pair_a_max.parse::<i32>().unwrap();
    let pair_a:Vec<i32> = (pair_a_min..=pair_a_max).collect();

    let (pair_b_min, pair_b_max) = pair_b.trim().split_once("-").unwrap();
    let pair_b_min = pair_b_min.parse::<i32>().unwrap();
    let pair_b_max = pair_b_max.parse::<i32>().unwrap();
    let pair_b:Vec<i32> = (pair_b_min..=pair_b_max).collect();

    return (pair_a, pair_b);
}

fn has_full_intersection(pairs: (Vec<i32>, Vec<i32>)) -> bool {
    let (pair_a, pair_b) = pairs;

    if pair_b.iter().all(|item| pair_a.contains(item)) {
        return true
    }

    if pair_a.iter().all(|item| pair_b.contains(item)) {
        return true
    }

    return false;
}

fn has_any_intersection(pairs: (Vec<i32>, Vec<i32>)) -> bool {
    let (pair_a, pair_b) = pairs;

    if pair_b.iter().any(|item| pair_a.contains(item)) {
        return true
    }

    if pair_a.iter().any(|item| pair_b.contains(item)) {
        return true
    }

    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_the_answer_for_part_one() {
        let content = std::fs::read_to_string("./fixtures/input_test.txt").unwrap();
        assert_eq!(2, part_one(&content));
    }

    #[test]
    fn it_returns_the_answer_for_part_two() {
        let content = std::fs::read_to_string("./fixtures/input_test.txt").unwrap();
        assert_eq!(4, part_two(&content));
    }
}
