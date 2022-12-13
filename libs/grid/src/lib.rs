use defaultmap::DefaultHashMap;
use derive_more::{Add, AddAssign, Sub, SubAssign, Sum};
use std::hash::Hash;
use std::iter::from_fn;

#[derive(
    Eq, PartialEq, Hash, Debug, Copy, Clone, AddAssign, SubAssign, Add, Sub, Sum, PartialOrd, Ord,
)]
pub struct Pos(isize, isize);

impl Pos {
    pub const NEIGHBORS: [Self; 4] = [Self(0, 1), Self(1, 0), Self(-1, 0), Self(0, -1)];

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
