pub use debug_print::{debug_eprint, debug_eprintln, debug_print, debug_println};
pub use defaultmap::DefaultHashMap;
pub use derive_more::{Add, AddAssign, Sub, SubAssign, Sum};
pub use hashbrown::{HashMap, HashSet};
pub use itertools::Itertools;
pub use num::*;
pub use pathfinding::directed::count_paths::count_paths;
pub use pathfinding::prelude::{bfs,dijkstra};
pub use rayon::prelude::*;
pub use regex::Regex;
pub use std::cmp;
pub use std::cmp::Reverse;
pub use std::collections::{BinaryHeap, VecDeque};
pub use std::fmt::{Debug, Display};
pub use std::time::Instant;

pub mod arena;
pub mod arena3d;
pub mod grid;

use std::hash::Hash;
use std::path::Path;

#[cfg(debug_assertions)]
pub const DEBUG: bool = true;

#[cfg(not(debug_assertions))]
pub const DEBUG: bool = false;

pub fn load_input<P: AsRef<Path>>(path: P) -> String {
    std::fs::read_to_string(path).unwrap()
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

pub trait ToInt {
    fn int(&self) -> i64;
    fn uint(&self) -> usize;
}

impl<T: Display> ToInt for T {
    fn int(&self) -> i64 {
        self.to_string().parse().unwrap()
    }

    fn uint(&self) -> usize {
        self.to_string().parse().unwrap()
    }
}
