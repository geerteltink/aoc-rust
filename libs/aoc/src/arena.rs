use defaultmap::DefaultHashMap;
use derive_more::{Add, AddAssign, Sub, SubAssign, Sum};
// use std::cmp::Ordering;
use std::collections::HashSet;
use std::hash::Hash;
use std::iter::from_fn;

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

    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

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

    pub fn walk(self, unit: Self) -> impl Iterator<Item = Self> {
        let mut pos = self;

        return from_fn(move || {
            pos += unit;
            Some(pos)
        });
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

/*
impl Ord for Loc {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.y, self.x).cmp(&(other.y, other.x))
    }
}
*/

pub struct Arena {
    pub map: HashSet<Loc>,
}

impl Arena {
    pub fn new() -> Self {
        Self {
            map: HashSet::new(),
        }
    }

    pub fn from_map(map: HashSet<Loc>) -> Self {
        Self { map }
    }

    pub fn insert(&mut self, loc: Loc) {
        self.map.insert(loc);
    }
}

//pub type ArenA<T> = DefaultHashMap<Loc, T>;

pub fn default_hash_map_arena_from_input<T: Clone>(
    s: &str,
    mut f: impl FnMut(char) -> T,
    default: T,
) -> DefaultHashMap<Loc, T> {
    let mut arena = DefaultHashMap::new(default);

    for (y, line) in s.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            arena[Loc::new(x as _, y as _)] = f(c);
        }
    }

    return arena;
}

pub fn hash_set_arena_from_input(s: &str, mut f: impl FnMut(char) -> bool) -> HashSet<Loc> {
    let mut arena = HashSet::new();

    for (y, line) in s.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if f(c) {
                arena.insert(Loc::new(x as _, y as _));
            }
        }
    }

    return arena;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_all_neighbors() {
        let loc = Loc::new(0, 0);

        assert_eq!(Loc::NEIGHBORS, loc.neighbors());
    }

    #[test]
    fn it_creates_a_new_arena() {
        let mut arena = Arena::new();
        let loc = Loc::new(0, 0);

        arena.insert(loc);

        assert_eq!(true, arena.map.contains(&loc));
    }
}
