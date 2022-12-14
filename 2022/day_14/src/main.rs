#![allow(unused_imports)]

use aoc::*;
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
    let mut grid: Grid<Pos, Sprite> = Grid::new(Sprite::Air);

    // Load map
    for path in input.lines() {
        let mut points = path
            .split("->")
            .map(|c| c.trim())
            .map(|c| c.split(','))
            .map(|mut c| {
                Pos(
                    c.next().unwrap().parse::<isize>().unwrap(),
                    c.next().unwrap().parse::<isize>().unwrap(),
                )
            });

        let mut from;
        let mut to = points.next().unwrap();
        for next in points {
            from = to;
            to = next;
            for x in from.0.min(to.0)..=from.0.max(to.0) {
                for y in from.1.min(to.1)..=from.1.max(to.1) {
                    grid.insert(Pos(x, y), Sprite::Rock);
                }
            }
        }
    }

    let mut count = 0;
    let max: isize = grid.iter().map(|c| c.0 .1).max().unwrap();

    // If it cannot move anymore, next sand unit
    while let Some(pos) = drop_sand(&grid, max, false) {
        grid.insert(pos, Sprite::Sand);
        count += 1;
    }

    grid_print(&grid);

    return count;
}

fn part_two(input: &String) -> usize {
    let mut grid: Grid<Pos, Sprite> = Grid::new(Sprite::Air);

    // Load map
    for path in input.lines() {
        let mut points = path
            .split("->")
            .map(|c| c.trim())
            .map(|c| c.split(','))
            .map(|mut c| {
                Pos(
                    c.next().unwrap().parse::<isize>().unwrap(),
                    c.next().unwrap().parse::<isize>().unwrap(),
                )
            });

        let mut from;
        let mut to = points.next().unwrap();
        for next in points {
            from = to;
            to = next;
            for x in from.0.min(to.0)..=from.0.max(to.0) {
                for y in from.1.min(to.1)..=from.1.max(to.1) {
                    grid.insert(Pos(x, y), Sprite::Rock);
                }
            }
        }
    }

    let mut count = 0;
    let max: isize = grid.iter().map(|c| c.0 .1).max().unwrap();

    // If it cannot move anymore, next sand unit
    while let Some(pos) = drop_sand(&grid, max, true) {
        grid.insert(pos, Sprite::Sand);
        count += 1;
    }

    grid_print(&grid);

    return count;
}

fn drop_sand(grid: &Grid<Pos, Sprite>, max_y: isize, part2: bool) -> Option<Pos> {
    let mut sand = Pos(500, 0);
    let floor = max_y + 2;

    if grid.get(&Pos(500, 0)) != &Sprite::Air {
        return None;
    }

    // Move sand unit, first down, then left, then right
    while grid.get(Pos(sand.0, sand.1 + 1)) == &Sprite::Air
        || grid.get(&Pos(sand.0 - 1, sand.1 + 1)) == &Sprite::Air
        || grid.get(&Pos(sand.0 + 1, sand.1 + 1)) == &Sprite::Air
    {
        if sand.1 == floor - 1 {
            if part2 {
                break;
            } else {
                return None
            }
        } else if grid.get(&Pos(sand.0, sand.1 + 1)) == &Sprite::Air {
            sand.1 += 1;
        } else if grid.get(&Pos(sand.0 - 1, sand.1 + 1)) == &Sprite::Air {
            sand.0 -= 1;
            sand.1 += 1;
        } else {
            sand.0 += 1;
            sand.1 += 1;
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
