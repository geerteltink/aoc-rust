use defaultmap::DefaultHashMap;
use derive_more::{Add, AddAssign, Sub, SubAssign, Sum};
pub use itertools::Itertools;
use std::hash::Hash;
use pathfinding::prelude::bfs;

// https://docs.rs/pathfinding/0.1.12/pathfinding/fn.bfs.html

static DAY: &'static str = "12";

#[derive(
    Eq, PartialEq, Hash, Debug, Copy, Clone, AddAssign, SubAssign, Add, Sub, Sum, PartialOrd, Ord,
)]
struct Pos(isize, isize);

impl Pos {
    fn neighbors (self) -> [Self; 4] {
        [Self(0, 1), Self(1, 0), Self(-1, 0), Self(0, -1)].map(|pos| self + pos)
    }
}

fn find_path(grid: &DefaultHashMap<Pos, u8>, start: Pos, end: u8) -> Option<usize> {
    let path = bfs(
        &start,
        |pos| {
            let neighbors = pos.neighbors();
            let cur = grid[pos];
            neighbors
                .into_iter()
                .filter(|pos| grid[pos] != b' ' && get_elevation(grid[pos]) - get_elevation(cur) <= 1)
                .collect_vec()
        },
        |pos| grid[pos] == end
    );
    
    return path.map(|x| x.len() - 1);
}

fn get_elevation(c: u8) -> i32 {
    if c == b'S' {
        return b'a' as i32;
    }
    
    if c == b'E' {
        return b'z' as i32;
    }
    
    return c as i32;
}

fn main() {
    let input = std::fs::read_to_string(format!("./2022/day_{DAY}/fixtures/input.txt")).unwrap();

    let result1 = part_one(&input);
    println!("Answer day {DAY} part one: {result1}");

    let result2 = part_two(&input);
    println!("Answer day {DAY} part one: {result2}");
}

fn part_one(input: &String) -> usize {
    let mut grid = DefaultHashMap::new(b' ');
    
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid[Pos(x as _, y as _)] = c as u8;
        }
    }
    
    // get start location
    let start = grid.iter().find(|loc| *loc.1 == b'S').unwrap();
    let end = b'E';
    // calculate minimal needed steps to reach end location
    let path = find_path(&grid, *start.0, end);
    let steps = path.unwrap();
    
    return steps;
}


fn part_two(input: &String) -> usize {
    let mut grid = DefaultHashMap::new(b' ');
    
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid[Pos(x as _, y as _)] = c as u8;
        }
    }    
    
    // iterate over grid
    // filter start locations and lowest locations
    // calculate steps for each start location
    // get the lowest steps
    let steps = grid
        .iter()
        .filter(|loc| *loc.1 == b'a' || *loc.1 == b'S')
        .filter_map(|loc| find_path(&grid, *loc.0, b'E'))
        .min()
        .unwrap();
    
    return steps;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_the_answer_for_part_one() {
        let input = std::fs::read_to_string("./fixtures/input_test.txt").unwrap();
        assert_eq!(31, part_one(&input));
    }

    #[test]
    fn it_returns_the_answer_for_part_two() {
        let input = std::fs::read_to_string("./fixtures/input_test.txt").unwrap();
        assert_eq!(29, part_two(&input));
    }
}
