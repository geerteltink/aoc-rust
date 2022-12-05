use core::panic;

static DAY: &'static str = "02";

fn main() {
    let input = std::fs::read_to_string(format!("./2022/day_{DAY}/fixtures/input.txt")).unwrap();

    let result1 = part_one(&input);
    println!("Answer day {DAY} part one: {result1}");

    let result2 = part_two(&input);
    println!("Answer day {DAY} part one: {result2}");
}

fn part_one(input: &String) -> i32 {
    // The score for a single round is the score for the shape you selected
    // (1 for Rock, 2 for Paper, and 3 for Scissors)
    // plus the score for the outcome of the round
    // (0 if you lost, 3 if the round was a draw, and 6 if you won).

    let mut score: i32 = 0;

    for line in input.lines() {
        let (player_a, player_b) = line.trim().split_once(" ").unwrap();
        let shape = shape_score(player_b) as i32;
        let game = game_score(player_a, player_b) as i32;
        score += shape + game;
    }

    return score;
}

fn part_two(_input: &String) -> i32 {
    return 0;
}

enum Choice {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum Score {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

fn shape_score(shape: &str) -> Choice {
    match shape {
        "X" => Choice::Rock,
        "Y" => Choice::Paper,
        "Z" => Choice::Scissors,
        _ => panic!("invalid choice"),
    }
}

fn game_score(player_a: &str, player_b: &str) -> Score {
    match [player_a, player_b] {
        ["A", "Y"] | ["B", "Z"] | ["C", "X"] => Score::Win,
        ["A", "X"] | ["B", "Y"] | ["C", "Z"] => Score::Draw,
        _ => Score::Loss,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_the_answer_for_part_one() {
        let input = std::fs::read_to_string("./fixtures/input_test.txt").unwrap();
        assert_eq!(15, part_one(&input));
    }

    #[ignore]
    #[test]
    fn it_returns_the_answer_for_part_two() {
        let input = std::fs::read_to_string("./fixtures/input_test.txt").unwrap();
        assert_eq!(12, part_two(&input));
    }
}
