use std::iter::from_fn;
use defaultmap::DefaultHashMap;
use derive_more::{AddAssign, SubAssign, Add, Sub, Sum};
use std::hash::Hash;
pub use itertools::Itertools;

static DAY: &'static str = "08";

#[derive(Eq, PartialEq, Hash, Debug, Copy, Clone, AddAssign, SubAssign, Add, Sub, Sum, PartialOrd, Ord)]
struct Pos(isize, isize);

impl Pos {
    pub fn x(self) -> isize {
        self.0
    }

    pub fn y(self) -> isize {
        self.1
    }
    
    pub fn walk(self, unit: Self) -> impl Iterator<Item = Self> {
        let mut pos = self;
        
        return from_fn(move || {
            pos += unit;
            Some(pos)
        });
    }
}

pub trait ExtraItertools: Iterator + Sized {
    fn test(
        self,
        mut pass: impl FnMut(&Self::Item) -> bool,
        mut fail: impl FnMut(&Self::Item) -> bool,
    ) -> bool {
        for item in self {
            if pass(&item) {
                return true;
            }
            if fail(&item) {
                return false;
            }
        }
        unreachable!("the iterator does not pass or fail");
    }
}

impl<T: Iterator + Sized> ExtraItertools for T {}

fn main() {
    let input = std::fs::read_to_string(format!("./2022/day_{DAY}/fixtures/input.txt")).unwrap();

    let result1 = part_one(&input);
    println!("Answer day {DAY} part one: {result1}");

    let result2 = part_two(&input);
    println!("Answer day {DAY} part one: {result2}");
}

fn part_one(input: &String) -> i32 {
    let mut grid = DefaultHashMap::new(-1i8);

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid[Pos(x as _, y as _)] = c.to_string().parse::<i8>().unwrap();
        }
    }
    
    let mut total_visible = 0;
    for (&pos, &height) in grid.iter() {
        let mut visible_from_outside = false;
        for direction in [Pos(0, 1), Pos(1, 0), Pos(-1, 0), Pos(0, -1)] {
            let visible = pos
                .walk(direction)
                .test(|p| grid[p] == -1, |p| grid[p] >= height);

            if visible {
                visible_from_outside = true;
            }
        }
        
        if visible_from_outside {
            total_visible += 1;
        }
    }
    
    //println!("{:#?}", grid);
    //println!("{:?}", total_visible);
    
    return total_visible;
}

fn part_two(input: &String) -> usize {
    let mut grid = DefaultHashMap::new(-1i8);

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid[Pos(x as _, y as _)] = c.to_string().parse::<i8>().unwrap();
        }
    }
    let mut max = 0;
    for (&pos, &height) in grid.iter() {
        let mut scenic_score = 1;
        for direction in [Pos(0, 1), Pos(1, 0), Pos(-1, 0), Pos(0, -1)] {
            let visible = pos
                .walk(direction)
                .test(|p| grid[p] == -1, |p| grid[p] >= height);
            
            let mut distance = pos
                .walk(direction)
                .take_while(|p| grid[p] < height && grid[p] != -1)
                .count();

            if !visible {
                distance += 1;
            }

            scenic_score *= distance;
        }
        
        max = scenic_score.max(max);
    }
    
    //println!("{:#?}", grid);
    //println!("{:?}", total_visible);
    
    return max;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_the_answer_for_part_one() {
        let input = std::fs::read_to_string("./fixtures/input_test.txt").unwrap();
        assert_eq!(21, part_one(&input));
    }

    #[test]
    fn it_returns_the_answer_for_part_two() {
        let input = std::fs::read_to_string("./fixtures/input_test.txt").unwrap();
        assert_eq!(8, part_two(&input));
    }
}
