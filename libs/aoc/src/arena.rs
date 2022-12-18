use derive_more::{Add, AddAssign, Sub, SubAssign, Sum};
use std::collections::HashSet;
use std::hash::Hash;

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
        Self { map,}
    }
    
    pub fn insert(&mut self, loc: Loc) {
        self.map.insert(loc);
    }
}
