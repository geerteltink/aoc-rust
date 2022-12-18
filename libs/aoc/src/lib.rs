pub use debug_print::{debug_eprint, debug_eprintln, debug_print, debug_println};
pub use itertools::Itertools;
pub use std::collections::HashSet;

use defaultmap::DefaultHashMap;
use derive_more::{Add, AddAssign, Sub, SubAssign, Sum};
use std::any::Any;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::from_fn;
use std::path::Path;

#[cfg(debug_assertions)]
pub const DEBUG: bool = true;

#[cfg(not(debug_assertions))]
pub const DEBUG: bool = false;

pub fn load_input<P: AsRef<Path>>(path: P) -> String {
    std::fs::read_to_string(path).unwrap()
}

pub trait ExtraItertools: Iterator + Sized {
    fn collect_set(self) -> HashSet<Self::Item>
    where
        Self::Item: Eq + Hash,
    {
        self.collect()
    }

    fn collect_string(self) -> String
    where
        String: FromIterator<Self::Item>,
    {
        self.collect()
    }

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

#[derive(
    Eq, PartialEq, Hash, Debug, Copy, Clone, AddAssign, SubAssign, Add, Sub, Sum, PartialOrd, Ord,
)]
pub struct Loc {
    pub x: i64,
    pub y: i64,
}

impl Loc {
    pub const NEIGHBORS: [Self; 4] = [
        Self { x: 0, y: 1 },
        Self { x: 1, y: 0 },
        Self { x: -1, y: 0 },
        Self { x: 0, y: -1 },
    ];

    pub fn neighbors(self) -> [Self; 4] {
        Self::NEIGHBORS.map(|loc| self + loc)
    }

    pub fn up(self, n: i64) -> Self {
        self + Self { x: 0, y: -1 * n }
    }

    pub fn down(self, n: i64) -> Self {
        self + Self { x: 0, y: 1 * n }
    }

    pub fn left(self, n: i64) -> Self {
        self + Self { x: -1 * n, y: 0 }
    }

    pub fn right(self, n: i64) -> Self {
        self + Self { x: 1 * n, y: 0 }
    }

    pub fn manhattan_distance(self) -> i64 {
        self.x.abs() + self.y.abs()
    }

    pub fn manhattan_distance_corners(self) -> i64 {
        self.y.abs().max(self.x.abs())
    }

    pub fn manhattan_circle(self, radius: i64) -> Vec<Loc> {
        let mut circle = Vec::new();
        for i in 0..radius {
            circle.push(Loc {
                x: self.x - radius + i,
                y: self.y + i,
            });
            circle.push(Loc {
                x: self.x + i,
                y: self.y + radius - i,
            });
            circle.push(Loc {
                x: self.x + radius - i,
                y: self.y - i,
            });
            circle.push(Loc {
                x: self.x - i,
                y: self.y - (radius - i),
            });
        }
        circle
    }
}

pub type Map<Loc, T> = DefaultHashMap<Loc, T>;

#[derive(
    Eq, PartialEq, Hash, Debug, Copy, Clone, AddAssign, SubAssign, Add, Sub, Sum, PartialOrd,
)]
pub struct Pos(pub isize, pub isize);

impl Pos {
    pub const NEIGHBORS: [Self; 4] = [Self(0, 1), Self(1, 0), Self(-1, 0), Self(0, -1)];

    pub fn x(self) -> isize {
        return self.0;
    }

    pub fn y(self) -> isize {
        return self.1;
    }

    pub fn neighbors(self) -> [Self; 4] {
        Self::NEIGHBORS.map(|pos| self + pos)
    }

    pub fn walk(self, unit: Self) -> impl Iterator<Item = Self> {
        let mut pos = self;

        return from_fn(move || {
            pos += unit;
            Some(pos)
        });
    }
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.1, self.0).cmp(&(other.1, other.0))
    }
}

/// Usage: Grid::new(-1i8);
pub type Grid<Pos, T> = DefaultHashMap<Pos, T>;

trait DefaultGridHashMap {
    fn print(&self);

    /*
    fn from_input<K, F: ?Sized, V>(input: &str, f: F, default: V) -> DefaultHashMap<K, V>
    where
        K: Eq + Hash,
        F: FnMut(char) -> V,
        V: Clone;
    */
}

impl<K: Eq + Hash, V: Clone> DefaultGridHashMap for DefaultHashMap<K, V> {
    fn print(&self) {
        println!("Printing: {}", 1);
    }

    /*
    fn from_input<F>(input: &str, f: F, default: V) -> DefaultHashMap<K, V>
    where
        K: Eq + Hash,
        F: FnMut(char) -> V,
        V: Clone,
        Pos: Borrow<K>
    {
        let mut grid = Self::new(default);

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.trim().chars().enumerate() {
                grid[Pos(x as _, y as _)] = f(c);
            }
        }

        return grid;
    }
    */
}

/// Usage: parse_grid(&input, |c| c as u8, b' ');
pub fn grid_from_input<F, T>(input: &str, mut f: F, default: T) -> Grid<Pos, T>
where
    F: FnMut(char) -> T,
    T: Clone,
{
    let mut grid = Grid::new(default);

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            grid[Pos(x as _, y as _)] = f(c);
        }
    }

    return grid;
}

pub fn grid_print<T: Clone + Debug + Any>(grid: &Grid<Pos, T>) {
    let min_x = grid.keys().map(|x| x.0).min().unwrap();
    let max_x = grid.keys().map(|x| x.0).max().unwrap();
    let min_y = grid.keys().map(|x| x.1).min().unwrap();
    let max_y = grid.keys().map(|x| x.1).max().unwrap();

    println!("printing grid (len={})", grid.len());

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let c = Pos(x, y);
            let data = format!("{:?}", grid[c]);
            if data.starts_with('\'') && data.ends_with('\'') {
                print!("{}", data.chars().rev().nth(1).unwrap());
            } else if data.starts_with('\"') && data.ends_with('\"') {
                let mut data = data.chars().skip(1).collect_vec();
                data.pop();
                print!("{}", data.into_iter().collect_string());
            } else {
                print!("{}", data.chars().next().unwrap());
            }
        }
        println!();
    }
    println!();
}

pub fn extract_numbers<const N: usize>(s: &str) -> [i64; N] {
    s.split(|c: char| !c.is_numeric() && c != '-')
        .filter_map(|x| {
            if x.is_empty() {
                None
            } else {
                Some(x.trim().to_string().parse().unwrap())
            }
        })
        .collect_vec()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_all_neighbors() {
        let pos = Pos(0, 0);

        assert_eq!(Pos::NEIGHBORS, pos.neighbors());
    }

    #[test]
    fn it_creates_a_new_grid() {
        let grid = Grid::new(-1i8);

        assert_eq!(grid[Pos(0, 0)], -1i8);
    }

    #[test]
    fn it_can_convert_input_data_into_a_grid() {
        let input = String::from(
            r#"Sabqponm
            abcryxxl
            accszExk
            acctuvwj
            abdefghi"#,
        );
        let grid = grid_from_input(&input, |c| c as u8, b' ');

        grid.print();

        assert_eq!(40, grid.len());
    }
}
