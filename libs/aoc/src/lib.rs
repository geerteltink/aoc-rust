pub use debug_print::{debug_eprint, debug_eprintln, debug_print, debug_println};
pub use itertools::Itertools;
pub use std::collections::HashSet;
pub use defaultmap::DefaultHashMap;

pub mod arena;
pub mod arena3d;

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
