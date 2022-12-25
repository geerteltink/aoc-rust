use derive_more::{Add, AddAssign, Sub, SubAssign, Sum};
use std::collections::HashSet;
use std::hash::Hash;

#[derive(
    Eq, PartialEq, Hash, Debug, Copy, Clone, AddAssign, SubAssign, Add, Sub, Sum, PartialOrd, Ord,
)]
pub struct Loc3D {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Loc3D {
    pub const NEIGHBORS: [Self; 6] = [
        Self { x: 1, y: 0, z: 0 },
        Self { x: 0, y: 1, z: 0 },
        Self { x: 0, y: 0, z: 1 },
        Self { x: -1, y: 0, z: 0 },
        Self { x: 0, y: -1, z: 0 },
        Self { x: 0, y: 0, z: -1 },
    ];

    pub fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }

    pub fn neighbors(self) -> [Self; 6] {
        Self::NEIGHBORS.map(|loc| self + loc)
    }
}

pub struct Arena3D {
    pub map: HashSet<Loc3D>,
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
    min_z: i64,
    max_z: i64,
}

impl Arena3D {
    pub fn new() -> Self {
        Self {
            map: HashSet::new(),
            min_x: 0,
            max_x: 0,
            min_y: 0,
            max_y: 0,
            min_z: 0,
            max_z: 0,
        }
    }

    pub fn from_map(map: HashSet<Loc3D>) -> Self {
        let min_x = map.iter().map(|x| x.x).min().unwrap() - 1;
        let max_x = map.iter().map(|x| x.x).max().unwrap() + 1;

        let min_y = map.iter().map(|x| x.y).min().unwrap() - 1;
        let max_y = map.iter().map(|x| x.y).max().unwrap() + 1;

        let min_z = map.iter().map(|x| x.z).min().unwrap() - 1;
        let max_z = map.iter().map(|x| x.z).max().unwrap() + 1;
        Self {
            map,
            min_x,
            max_x,
            min_y,
            max_y,
            min_z,
            max_z,
        }
    }

    pub fn insert(&mut self, loc: Loc3D) {
        self.map.insert(loc);

        self.min_x = self.map.iter().map(|x| x.x).min().unwrap() - 1;
        self.max_x = self.map.iter().map(|x| x.x).max().unwrap() + 1;
        self.min_y = self.map.iter().map(|x| x.y).min().unwrap() - 1;
        self.max_y = self.map.iter().map(|x| x.y).max().unwrap() + 1;
        self.min_z = self.map.iter().map(|x| x.z).min().unwrap() - 1;
        self.max_z = self.map.iter().map(|x| x.z).max().unwrap() + 1;
    }

    pub fn is_in_bounds(&self, loc: &Loc3D) -> bool {
        loc.x >= self.min_x
            && loc.x <= self.max_x
            && loc.y >= self.min_y
            && loc.y <= self.max_y
            && loc.z >= self.min_z
            && loc.z <= self.max_z
    }

    pub fn is_adjacent(&self, loc: &Loc3D) -> bool {
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    let new_point = Loc3D::new(loc.x + dx, loc.y + dy, loc.z + dz);

                    if self.map.contains(&new_point) {
                        return true;
                    }
                }
            }
        }

        false
    }
}
