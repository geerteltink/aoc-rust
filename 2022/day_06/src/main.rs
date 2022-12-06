static DAY: &'static str = "06";

use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string(format!("./2022/day_{DAY}/fixtures/input.txt")).unwrap();

    let result1 = part_one(&input);
    println!("Answer day {DAY} part one: {result1}");

    let result2 = part_two(&input);
    println!("Answer day {DAY} part one: {result2}");
}

fn part_one(input: &String) -> usize {
    let message = input.chars().collect_vec();
    
    return get_start_of_packet(message, 4);
}

fn part_two(input: &String) -> usize {
    let message = input.chars().collect_vec();
    
    return get_start_of_packet(message, 14);
}

fn get_start_of_packet(message: Vec<char>, packet_size: usize) -> usize {
    let mut count = 0;
    for marker in message.windows(packet_size) { // get slice of x characters, if smaller than x, it is ignored
        if marker.iter().all_unique() { // check if all characters are unique
            break;
        }
        count += 1;
    }
    
    return count + packet_size;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_the_answer_for_part_one() {
        let input = std::fs::read_to_string("./fixtures/input_test.txt").unwrap();
        assert_eq!(10, part_one(&input));
    }

    #[test]
    fn it_returns_the_answer_for_part_two() {
        let input = std::fs::read_to_string("./fixtures/input_test.txt").unwrap();
        assert_eq!(29, part_two(&input));
    }
}
