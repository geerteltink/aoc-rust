use defaultmap::DefaultHashMap;
use derive_more::{Add, AddAssign, Sub, SubAssign, Sum};
pub use itertools::Itertools;
use std::hash::Hash;

static DAY: &'static str = "09";

#[derive(
    Eq, PartialEq, Hash, Debug, Copy, Clone, AddAssign, SubAssign, Add, Sub, Sum, PartialOrd, Ord,
)]
struct Pos(isize, isize);

impl Pos {
    pub fn up(self) -> Self {
        Self(self.0, self.1 + 1)
    }

    pub fn down(self) -> Self {
        Self(self.0, self.1 - 1)
    }
    
    pub fn right(self) -> Self {
        Self(self.0 + 1, self.1)
    }
    
    pub fn left(self) -> Self {
        Self(self.0 - 1, self.1)
    }
    
    pub fn follow(self, other: Self, direction: &str) -> Self {
        let new = self - other;
        let mut changed = self.clone();
        
        match new {
            Pos(2, -1) => {
                changed = self.left();
                changed = changed.up();
            },
            Pos(2, 1) => {
                changed = self.left();
                changed = changed.down();
            },
            Pos(2, _) => changed = self.left(),
            Pos(-2, -1) => {
                changed = self.right();
                changed = changed.up();
            },
            Pos(-2, 1) => {
                changed = self.right();
                changed = changed.down();
            },
            Pos(-2, _) => changed = self.right(),
            Pos(-1, 2) => {
                changed = self.down();
                changed = changed.right();
            },
            Pos(1, 2) => {
                changed = self.down();
                changed = changed.left();
            },
            Pos(_, 2) => changed = self.down(),
            Pos(-1, -2) => {
                changed = self.up();
                changed = changed.right();
            },
            Pos(1, -2) => {
                changed = self.up();
                changed = changed.left();
            },
            Pos(_, -2) => changed = self.up(),
            Pos(_, _) => changed = self,
        };
        
        println!("H{:?} T{:?} -> {:?} | {:?}: {:?}", other, self, direction, new, changed);

        return changed;
    }
}

fn main() {
    let input = std::fs::read_to_string(format!("./2022/day_{DAY}/fixtures/input.txt")).unwrap();

    let result1 = part_one(&input);
    println!("Answer day {DAY} part one: {result1}");

    let result2 = part_two(&input);
    println!("Answer day {DAY} part one: {result2}");
}

fn part_one(input: &String) -> usize {
    let mut visited = DefaultHashMap::new(-1i8);
    
    // starting position
    let mut head = Pos(0,0);
    let mut tail = Pos(0,0);
    visited[tail] = 1;
    
    // handle motions
    for motion in input.lines() {
        let (direction, moves) = motion.trim().split_once(" ").unwrap();
        let steps = moves.parse::<i32>().unwrap();

        // handle each single step in a motion
        for _ in 0..steps {
            match direction {
                "U" => {
                    head = head.up();
                    tail = tail.follow(head, direction);
                },
                "D" => {
                    head = head.down();
                    tail = tail.follow(head, direction);
                },
                "L" => {
                    head = head.left();
                    tail = tail.follow(head, direction);
                },
                "R" => {
                    head = head.right();
                    tail = tail.follow(head, direction);
                },
                _ => panic!("invalid direction"),
            }
            
            // record the places the tail visited
            visited[tail] = 1;
        }
    }

    return visited.len();
}

fn part_two(_input: &String) -> i32 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_the_answer_for_part_one() {
        let input = std::fs::read_to_string("./fixtures/input_test.txt").unwrap();
        assert_eq!(13, part_one(&input));
    }

    #[ignore]
    #[test]
    fn it_returns_the_answer_for_part_two() {
        let input = std::fs::read_to_string("./fixtures/input_test.txt").unwrap();
        assert_eq!(0, part_two(&input));
    }
}
