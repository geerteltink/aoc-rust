use defaultmap::DefaultHashMap;
use derive_more::{Add, AddAssign, Sub, SubAssign, Sum};
use std::iter::from_fn;

#[derive(
    Eq, PartialEq, Hash, Debug, Copy, Clone, AddAssign, SubAssign, Add, Sub, Sum, PartialOrd, Ord,
)]
pub struct Pos(isize, isize);

impl Pos {
    pub const NEIGHBORS: [Self; 4] = [Self(0, 1), Self(1, 0), Self(-1, 0), Self(0, -1)];
    
    pub fn neighbors (self) -> [Self; 4] {
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

/// Usage: parse_grid(&input, |c| c as u8, b' ');
pub fn grid_from_input<T: Clone>(input: &str, mut f: impl FnMut(char) -> T, default: T) -> Grid<Pos, T> {
    let mut grid = DefaultHashMap::new(default);

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
        let pos = Pos(0,0);
        assert_eq!(Pos::NEIGHBORS, pos.neighbors());
    }
    
    #[test]
    fn it_creates_a_new_grid() {
        let grid: DefaultHashMap<Pos, i8> = DefaultHashMap::new(-1i8);
        assert_eq!(grid, Grid::new(-1i8));
    }
    
    #[test]
    fn it_sets_a_default() {
        let grid: DefaultHashMap<Pos, i8> = DefaultHashMap::new(-1i8);
        assert_eq!(-1i8, grid[Pos(0,0)]);
    }
    
    #[test]
    fn it_can_convert_input_data_into_a_grid() {
        let input= String::from(r#"Sabqponm
            abcryxxl
            accszExk
            acctuvwj
            abdefghi"#);
        let grid = grid_from_input(&input, |c| c as u8, b' ');
        
        assert_eq!(40, grid.len());
    }
}
