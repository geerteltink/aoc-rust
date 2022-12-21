static DAY: &'static str = "06";

use aoc::*;

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
    let mut start = 0;

    // get slice of x characters, if smaller than x, it is ignored
    for marker in message.windows(packet_size) {
        // check if all characters are unique
        if marker.iter().all_unique() {
            break;
        }
        start += 1;
    }

    return start + packet_size;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_the_answer_for_part_one() {
        let input = std::fs::read_to_string("./fixtures/input_test.txt").unwrap();
        assert_eq!(10, part_one(&input));
        
        assert_eq!(part_one(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string()), 7);
        assert_eq!(part_one(&"bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()), 5);
        assert_eq!(part_one(&"nppdvjthqldpwncqszvftbrmjlhg".to_string()), 6);
        assert_eq!(part_one(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()), 10);
        assert_eq!(part_one(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()), 11);
    }

    #[test]
    fn it_returns_the_answer_for_part_two() {
        let input = std::fs::read_to_string("./fixtures/input_test.txt").unwrap();
        assert_eq!(29, part_two(&input));
        
        assert_eq!(part_two(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string()), 19);
        assert_eq!(part_two(&"bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()), 23);
        assert_eq!(part_two(&"nppdvjthqldpwncqszvftbrmjlhg".to_string()), 23);
        assert_eq!(part_two(&"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()), 29);
        assert_eq!(part_two(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()), 26);
    }
}
