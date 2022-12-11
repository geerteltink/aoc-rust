use std::{collections::HashSet, fmt::Debug, hash::Hash};
use Direction::*;

static DAY: &'static str = "09";

fn main() {
    let input = std::fs::read_to_string(format!("./2022/day_{DAY}/fixtures/input.txt")).unwrap();

    let result1 = part_one(&input);
    println!("Answer day {DAY} part one: {result1}");

    let result2 = part_two(&input);
    println!("Answer day {DAY} part one: {result2}");
}

fn part_one(input: &String) -> usize {
    let mut rope = Rope::new(2);
    let mut visited = HashSet::new();
    visited.insert(rope.end());
    
    // handle motions
    for motion in input.lines() {
        let (direction, moves) = motion.trim().split_once(" ").unwrap();
        let direction = Direction::from(direction);
        let steps: i32 = moves.parse().unwrap();
        
        for _ in 0..steps {
            rope.motion(direction);            
            visited.insert(rope.end());
        }
    }

    return visited.len();
}

fn part_two(input: &String) -> usize {
    let mut rope = Rope::new(10);
    let mut visited = HashSet::new();
    visited.insert(rope.end());
    
    // handle motions
    for motion in input.lines() {
        let (direction, moves) = motion.trim().split_once(" ").unwrap();
        let direction = Direction::from(direction);
        let steps: i32 = moves.parse().unwrap();
        
        for _ in 0..steps {
            rope.motion(direction);            
            visited.insert(rope.end());
        }
    }

    return visited.len();
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl From<&str> for Direction {
    fn from(s: &str) -> Self {
        match s {
            "U" => Up,
            "D" => Down,
            "R" => Right,
            "L" => Left,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Knot {
    x: i64,
    y: i64,
}

impl Knot {
    fn new(x: i64, y: i64) -> Self {
        Knot { x, y }
    }
}

#[derive(Debug)]
struct Rope {
    len: usize,
    knots: Vec<Knot>,
}

impl Rope {
    fn new(len: usize) -> Self {
        Rope {
            len,
            knots: vec![Knot::new(0, 0); len],
        }
    }
    
    fn motion(&mut self, direction: Direction) {
        match direction {
            Left => self.knots[0].x -= 1,
            Down => self.knots[0].y -= 1,
            Right => self.knots[0].x += 1,
            Up => self.knots[0].y += 1,
        }
        
        for i in 1..self.len {
            let prev = self.knots[i - 1];
            let next = self.knots[i];

            let (dx, dy) = (prev.x - next.x, prev.y - next.y);
            if dx.abs() > 1 || dy.abs() > 1 {
                self.knots[i].x += dx.signum();
                self.knots[i].y += dy.signum();
            }
        }
    }

    fn end(&self) -> Knot {
        *self.knots.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_the_answer_for_part_one() {
        let input = std::fs::read_to_string("./fixtures/input_test.txt").unwrap();
        assert_eq!(13, part_one(&input));
    }

    #[test]
    fn it_returns_the_answer_for_part_two() {
        let input = std::fs::read_to_string("./fixtures/input_test.txt").unwrap();
        assert_eq!(1, part_two(&input));
    }

    #[test]
    fn it_returns_the_answer_for_part_two_example_2() {
        let input = std::fs::read_to_string("./fixtures/input_test2.txt").unwrap();
        assert_eq!(36, part_two(&input));
    }
}
