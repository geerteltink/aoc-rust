#![allow(unused_imports)]

use aoc::*;
use aoc::arena::*;
use std::cmp::Ordering;

static DAY: &'static str = "14";

#[derive(Eq, PartialEq, Debug, Copy, Clone)]
enum Sprite {
    Air = -1,
    Rock = 1,
    Sand = 0,
}

fn main() {
    let input = load_input(format!("./2022/day_{DAY}/fixtures/input.txt"));

    let result1 = part_one(&input);
    println!("Answer day {DAY} part one: {result1}");

    let result2 = part_two(&input);
    println!("Answer day {DAY} part one: {result2}");
}

fn part_one(input: &String) -> usize {
    let mut grid: DefaultHashMap<Loc, Sprite> = DefaultHashMap::new(Sprite::Air);

    // Load map
    for path in input.lines() {
        let mut points = path
            .split("->")
            .map(|c| c.trim())
            .map(|c| c.split(','))
            .map(|mut c| {
                Loc::new(
                    c.next().unwrap().parse::<i64>().unwrap(),
                    c.next().unwrap().parse::<i64>().unwrap(),
                )
            });

        let mut from;
        let mut to = points.next().unwrap();
        for next in points {
            from = to;
            to = next;
            for x in from.x.min(to.x)..=from.x.max(to.x) {
                for y in from.y.min(to.y)..=from.y.max(to.y) {
                    grid.insert(Loc::new(x, y), Sprite::Rock);
                }
            }
        }
    }

    let mut count = 0;
    let max: i64 = grid.iter().map(|c| c.0.y).max().unwrap();

    // If it cannot move anymore, next sand unit
    while let Some(pos) = drop_sand(&grid, max, false) {
        grid.insert(pos, Sprite::Sand);
        count += 1;
    }

    return count;
}

fn part_two(input: &String) -> usize {
    let mut grid: DefaultHashMap<Loc, Sprite> = DefaultHashMap::new(Sprite::Air);

    // Load map
    for path in input.lines() {
        let mut points = path
            .split("->")
            .map(|c| c.trim())
            .map(|c| c.split(','))
            .map(|mut c| {
                Loc::new(
                    c.next().unwrap().parse::<i64>().unwrap(),
                    c.next().unwrap().parse::<i64>().unwrap(),
                )
            });

        let mut from;
        let mut to = points.next().unwrap();
        for next in points {
            from = to;
            to = next;
            for x in from.x.min(to.x)..=from.x.max(to.x) {
                for y in from.y.min(to.y)..=from.y.max(to.y) {
                    grid.insert(Loc::new(x, y), Sprite::Rock);
                }
            }
        }
    }

    let mut count = 0;
    let max: i64 = grid.iter().map(|c| c.0.y).max().unwrap();

    // If it cannot move anymore, next sand unit
    while let Some(pos) = drop_sand(&grid, max, true) {
        grid.insert(pos, Sprite::Sand);
        count += 1;
    }

    return count;
}

fn drop_sand(grid: &DefaultHashMap<Loc, Sprite>, max_y: i64, part2: bool) -> Option<Loc> {
    let mut sand = Loc::new(500, 0);
    let floor = max_y + 2;

    if grid.get(&Loc::new(500, 0)) != &Sprite::Air {
        return None;
    }

    // Move sand unit, first down, then left, then right
    while grid.get(Loc::new(sand.x, sand.y + 1)) == &Sprite::Air
        || grid.get(&Loc::new(sand.x - 1, sand.y + 1)) == &Sprite::Air
        || grid.get(&Loc::new(sand.x + 1, sand.y + 1)) == &Sprite::Air
    {
        if sand.y == (floor - 1) as i64 {
            if part2 {
                break;
            } else {
                return None
            }
        } else if grid.get(&Loc::new(sand.x, sand.y + 1)) == &Sprite::Air {
            sand.y += 1;
        } else if grid.get(&Loc::new(sand.x - 1, sand.y + 1)) == &Sprite::Air {
            sand.x -= 1;
            sand.y += 1;
        } else {
            sand.x += 1;
            sand.y += 1;
        }
    }
    Some(sand)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_the_answer_for_part_one() {
        let input = load_input("./fixtures/input_test.txt");
        assert_eq!(24, part_one(&input));
    }

    #[test]
    fn it_returns_the_answer_for_part_two() {
        let input = load_input("./fixtures/input_test.txt");
        assert_eq!(93, part_two(&input));
    }
}
