#![allow(unused_imports)]

use aoc::*;
use std::collections::HashSet;

static DAY: &'static str = "04";

fn main() {
    let input = load_input(format!("./2023/day_{DAY}/fixtures/input.txt"));

    let start = Instant::now();
    let result1 = part_one(&input);
    let end = Instant::now();
    println!("Elapsed time: {:?}", end - start);
    println!("Answer day {DAY} part one: {result1} in {:?}", end - start);

    let start = Instant::now();
    let result2 = part_two(&input);
    let end = Instant::now();
    println!("Answer day {DAY} part two: {result2} in {:?}", end - start);
}

fn part_one(input: &String) -> i32 {
    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    // card id: winning numbers | numbers you have

    // You have to figure out which of the numbers you have
    // appear in the list of winning numbers.
    // The first match makes the card worth one point
    // and each match after the first doubles the point value of that card.

    let cards: Vec<Card> = input.lines().map(|line| Card::from_string(line)).collect();

    let mut total_points = 0;
    for card in cards {
        let card_winning_numbers = card
            .winning_numbers
            .intersection(&card.card_numbers)
            .count();
        if card_winning_numbers > 0 {
            total_points += 2_i32.pow(card_winning_numbers as u32 - 1);
        }
    }

    return total_points;
}

fn part_two(input: &String) -> usize {
    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    // card id: winning numbers | numbers you have

    // Scratchcards only cause you to win more scratchcards
    // equal to the number of winning numbers you have.
    // Specifically, you win copies of the scratchcards below the winning
    // card equal to the number of matches.

    let cards: Vec<Card> = input.lines().map(|line| Card::from_string(line)).collect();

    let mut copies = cards.clone();
    for card in &cards {
        let card_winning_numbers = card
            .winning_numbers
            .intersection(&card.card_numbers)
            .count();
        let count = copies.iter().filter(|x| x.id == card.id).count();
        for x in card.id + 1..=card.id + card_winning_numbers {
            for _ in 0..count {
                copies.push(cards[x - 1].clone());
            }
        }
    }

    return copies.len();
}

#[derive(Debug, Clone)]
struct Card {
    id: usize,
    winning_numbers: HashSet<i32>,
    card_numbers: HashSet<i32>,
}

impl Card {
    fn from_string(line: &str) -> Card {
        let (card, numbers) = line.split_once(":").unwrap();
        let card_id = card
            .split_once(" ")
            .unwrap()
            .1
            .trim()
            .parse::<usize>()
            .unwrap();

        let (winning_numbers_string, card_numbers_string) = numbers.split_once("|").unwrap();
        let winning_numbers = winning_numbers_string
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let card_numbers = card_numbers_string
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        Card {
            id: card_id,
            winning_numbers: HashSet::from_iter(winning_numbers),
            card_numbers: HashSet::from_iter(card_numbers),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_the_test_answer_for_part_one() {
        let input = load_input("./fixtures/input_test.txt");
        assert_eq!(13, part_one(&input));
    }

    #[test]
    fn it_returns_the_answer_for_part_one() {
        let input = load_input("./fixtures/input.txt");
        assert_eq!(28750, part_one(&input));
    }

    #[test]
    fn it_returns_the_test_answer_for_part_two() {
        let input = load_input("./fixtures/input_test.txt");
        assert_eq!(30, part_two(&input));
    }

    #[test]
    fn it_returns_the_answer_for_part_two() {
        let input = load_input("./fixtures/input.txt");
        assert_eq!(10212704, part_two(&input));
    }
}
