static DAY: &'static str = "04";

fn main() {
    let input = std::fs::read_to_string(format!("./2022/day_{DAY}/fixtures/input.txt")).unwrap();

    let result1 = part_one(&input);
    println!("Answer day {DAY} part one: {result1}");

    let result2 = part_two(&input);
    println!("Answer day {DAY} part one: {result2}");
}

fn part_one(input: &String) -> i32 {
    let mut full_intersections: i32 = 0;
    for line in input.lines() {
        let pairs = get_pairs(line);
        if has_full_intersection(pairs) {
            full_intersections += 1;
        }
    }

    return full_intersections;
}

fn part_two(input: &String) -> i32 {
    let mut any_intersections: i32 = 0;
    for line in input.lines() {
        let pairs = get_pairs(line);
        if has_any_intersection(pairs) {
            any_intersections += 1;
        }
    }

    return any_intersections;
}

fn get_pairs(sections: &str) -> (Vec<i32>, Vec<i32>) {
    let (pair_a, pair_b) = sections.trim().split_once(",").unwrap();

    let pair_a: Vec<i32> = get_pair(pair_a);
    let pair_b: Vec<i32> = get_pair(pair_b);

    return (pair_a, pair_b);
}

fn get_pair(pair: &str) -> Vec<i32> {
    let (min, max) = pair.trim().split_once("-").unwrap();
    let min = min.parse::<i32>().unwrap();
    let max = max.parse::<i32>().unwrap();
    let pair: Vec<i32> = (min..=max).collect();

    return pair;
}

fn has_full_intersection(pairs: (Vec<i32>, Vec<i32>)) -> bool {
    let (pair_a, pair_b) = pairs;

    if pair_b.iter().all(|item| pair_a.contains(item)) {
        return true;
    }

    if pair_a.iter().all(|item| pair_b.contains(item)) {
        return true;
    }

    return false;
}

fn has_any_intersection(pairs: (Vec<i32>, Vec<i32>)) -> bool {
    let (pair_a, pair_b) = pairs;

    if pair_b.iter().any(|item| pair_a.contains(item)) {
        return true;
    }

    if pair_a.iter().any(|item| pair_b.contains(item)) {
        return true;
    }

    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_the_answer_for_part_one() {
        let input = std::fs::read_to_string("./fixtures/input_test.txt").unwrap();
        assert_eq!(2, part_one(&input));
    }

    #[test]
    fn it_returns_the_answer_for_part_two() {
        let input = std::fs::read_to_string("./fixtures/input_test.txt").unwrap();
        assert_eq!(4, part_two(&input));
    }

    #[test]
    fn it_converts_a_string_to_vectors() {
        let input = "2-6,4-7";
        let expected: (Vec<i32>, Vec<i32>) = (vec![2, 3, 4, 5, 6], vec![4, 5, 6, 7]);

        assert_eq!(expected, get_pairs(&input));
    }

    #[test]
    fn it_converts_a_string_to_vector() {
        let input = "2-6";
        let expected: Vec<i32> = vec![2, 3, 4, 5, 6];

        assert_eq!(expected, get_pair(&input));
    }

    #[test]
    fn it_has_not_full_intersection() {
        let pairs: (Vec<i32>, Vec<i32>) = (vec![2, 3, 4, 5, 6], vec![4, 5, 6, 7]);

        assert_eq!(false, has_full_intersection(pairs));
    }

    #[test]
    fn it_has_full_intersection() {
        let pairs = [
            (vec![2, 3, 4, 5, 6], vec![4, 5, 6]),
            (vec![2, 3, 4], vec![1, 2, 3, 4, 5, 6]),
        ];

        for pair in pairs {
            assert_eq!(true, has_full_intersection(pair));
        }
    }

    #[test]
    fn it_has_any_intersection() {
        let pairs = [
            (vec![5, 6, 7], vec![7, 8, 0]),
            (vec![2, 3, 4, 5, 6, 7, 8], vec![3, 4, 5, 6, 7]),
            (vec![6], vec![4, 5, 6]),
            (vec![2, 3, 4, 5, 6], vec![4, 5, 6, 7, 8]),
        ];

        for pair in pairs {
            assert_eq!(true, has_any_intersection(pair));
        }
    }
}
