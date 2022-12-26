use defaultmap::DefaultHashMap;
use std::any::Any;
use std::collections::HashSet;
use num::Integer;
use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use hashbrown::HashMap;
use itertools::Itertools;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Coordinate {
    pub x: isize,
    pub y: isize,
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Add for Coordinate {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Coordinate {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl AddAssign for Coordinate {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl SubAssign for Coordinate {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl Ord for Coordinate {
    // Reading order: Y then X
    fn cmp(&self, other: &Self) -> Ordering {
        (self.y, self.x).cmp(&(other.y, other.x))
    }
}

impl PartialOrd for Coordinate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Coordinate {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn neighbors(&self) -> Vec<Self> {
        vec![
            Coordinate {
                x: self.x - 1,
                y: self.y,
            },
            Coordinate {
                x: self.x + 1,
                y: self.y,
            },
            Coordinate {
                x: self.x,
                y: self.y - 1,
            },
            Coordinate {
                x: self.x,
                y: self.y + 1,
            },
        ]
    }

    pub fn extended_neighbors(&self) -> Vec<Self> {
        vec![
            Coordinate {
                x: self.x - 1,
                y: self.y - 1,
            },
            Coordinate {
                x: self.x - 1,
                y: self.y,
            },
            Coordinate {
                x: self.x - 1,
                y: self.y + 1,
            },
            Coordinate {
                x: self.x,
                y: self.y - 1,
            },
            Coordinate {
                x: self.x,
                y: self.y + 1,
            },
            Coordinate {
                x: self.x + 1,
                y: self.y - 1,
            },
            Coordinate {
                x: self.x + 1,
                y: self.y,
            },
            Coordinate {
                x: self.x + 1,
                y: self.y + 1,
            },
        ]
    }

    pub fn hex_neighbors(&self) -> Vec<Self> {
        vec![
            Coordinate {
                x: self.x - 1 - 1,
                y: self.y,
            },
            Coordinate {
                x: self.x + 1 + 1,
                y: self.y,
            },
            Coordinate {
                x: self.x + 1,
                y: self.y - 1,
            },
            Coordinate {
                x: self.x + 1,
                y: self.y + 1,
            },
            Coordinate {
                x: self.x - 1,
                y: self.y - 1,
            },
            Coordinate {
                x: self.x - 1,
                y: self.y + 1,
            },
        ]
    }

    pub fn manhattan_distance(&self, other: &Self) -> isize {
        self.x.max(other.x) - self.x.min(other.x) + self.y.max(other.y) - self.y.min(other.y)
    }

    /*
    pub fn manhattan_circle(self, radius: T) -> Vec<Coordinate> {
        let mut circle = Vec::new();
        for i in 0..radius {
            circle.push(Coordinate {
                x: self.x - radius + i,
                y: self.y + i,
            });
            circle.push(Coordinate {
                x: self.x + i,
                y: self.y + radius - i,
            });
            circle.push(Coordinate {
                x: self.x + radius - i,
                y: self.y - i,
            });
            circle.push(Coordinate {
                x: self.x - i,
                y: self.y - (radius - i),
            });
        }
        circle
    }
    */
}

pub type Grid<T> = DefaultHashMap<Coordinate, T>;

/// Usage:
///   let arena = create_grid_from_input(&input, |c| c, '#');
pub fn create_grid_from_input<T: Clone>(s: &str, mut f: impl FnMut(char) -> T, default: T) -> Grid<T> {
    let mut grid = Grid::new(default);

    for (y, line) in s.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid[Coordinate::new(x as isize, y as isize)] = f(c);
        }
    }

    return grid;
}

pub type GridSet = HashSet<Coordinate>;

/// Usage:
///   let mut arena = create_hash_set_from_input(&input, |c| c == '#');
pub fn create_hash_set_from_input(s: &str, mut f: impl FnMut(char) -> bool) -> GridSet {
    let mut grid = GridSet::new();

    for (y, line) in s.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if f(c) {
                grid.insert(Coordinate::new(x as isize, y as isize));
            }
        }
    }

    return grid;
}

pub fn print_hash_set_grid(grid: &HashSet<Coordinate>) {
    let min_x = grid.iter().map(|c| c.x).min().unwrap();
    let max_x = grid.iter().map(|c| c.x).max().unwrap();
    let min_y = grid.iter().map(|c| c.y).min().unwrap();
    let max_y = grid.iter().map(|c| c.y).max().unwrap();
    
    println!("printing grid (len={})", grid.len());

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let c = Coordinate::new(x, y);
            if grid.contains(&c) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_all_neighbors() {
        let coordinate = Coordinate::new(0, 0);

        let expected = vec![
            Coordinate::new(-1, 0),
            Coordinate::new(1, 0),
            Coordinate::new(0, -1),
            Coordinate::new(0, 1),
        ];

        assert_eq!(expected, coordinate.neighbors());
    }

    #[test]
    fn it_calculates_the_manhattan_distance() {
        let a = Coordinate::new(0, 0);
        let b = Coordinate::new(4, 8);

        assert_eq!(12, a.manhattan_distance(&b));
    }
}
