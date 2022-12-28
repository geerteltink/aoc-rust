use hashbrown::hash_map::*;
use hashbrown::HashMap;
use itertools::Itertools;
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::ops::{Add, AddAssign, Index, IndexMut, Mul, Neg, Sub, SubAssign};

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

impl Mul<isize> for Coordinate {
    type Output = Self;
    fn mul(self, rhs: isize) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Neg for Coordinate {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
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

    pub fn up(self, n: isize) -> Self {
        self + Self::new(0, -1) * n
    }

    pub fn down(self, n: isize) -> Self {
        self + Self::new(0, 1) * n
    }

    pub fn left(self, n: isize) -> Self {
        self + Self::new(-1, 0) * n
    }

    pub fn right(self, n: isize) -> Self {
        self + Self::new(1, 0) * n
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

pub struct GridDimensions {
    pub min_x: isize,
    pub max_x: isize,
    pub total_x: isize,
    pub min_y: isize,
    pub max_y: isize,
    pub total_y: isize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid<V: Clone> {
    map: HashMap<Coordinate, V>,
    default: V,
}

impl<V: Default + Clone> Default for Grid<V> {
    /// The `default()` constructor creates an empty Grid with the default of `V`
    /// as the default for missing keys.
    /// This is desired default for most use cases, if your case requires a
    /// different default you should use the `new()` constructor.
    fn default() -> Grid<V> {
        Grid {
            map: HashMap::default(),
            default: V::default(),
        }
    }
}

impl<V: Default + Clone> From<HashMap<Coordinate, V>> for Grid<V> {
    /// If you already have a `HashMap` that you would like to convert to a
    /// `Grid` you can use the `into()` method on the `HashMap` or the
    /// `from()` constructor of `Grid`.
    /// The default value for missing keys will be `V::default()`,
    /// if this is not desired `Grid::new_with_map()` should be used.
    fn from(map: HashMap<Coordinate, V>) -> Grid<V> {
        Grid {
            map,
            default: V::default(),
        }
    }
}

impl<V: Clone> Into<HashMap<Coordinate, V>> for Grid<V> {
    /// The into method can be used to convert a `Grid` back into a
    /// `HashMap`.
    fn into(self) -> HashMap<Coordinate, V> {
        self.map
    }
}

/// Implements the `Index` trait so you can do `map[key]`.
/// Nonmutable indexing can be done both by passing a reference or an owned value as the key.
impl<'a, V: Clone> Index<Coordinate> for Grid<V> {
    type Output = V;

    fn index(&self, index: Coordinate) -> &V {
        self.get(index)
    }
}

/// Implements the `IndexMut` trait so you can do `map[key] = val`.
/// Mutably indexing can only be done when passing an owned value as the key.
impl<V: Clone> IndexMut<Coordinate> for Grid<V> {
    #[inline]
    fn index_mut(&mut self, index: Coordinate) -> &mut V {
        self.get_mut(index)
    }
}

impl<V: Clone> Grid<V> {
    /// Creates an empty `Grid` with `default` as the default for missing keys.
    /// When the provided `default` is equivalent to `V::default()` it is preferred to use
    /// `Grid::default()` instead.
    pub fn new(default: V) -> Grid<V> {
        Grid {
            map: HashMap::new(),
            default,
        }
    }

    pub fn from_input(input: &str, mut f: impl FnMut(char) -> V, default: V) -> Grid<V> {
        let mut grid: Grid<V> = Grid::new(default);

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                grid[Coordinate::new(x as isize, y as isize)] = f(c);
            }
        }

        grid
    }

    pub fn from_filtered_input(
        input: &str,
        mut f: impl FnMut(char) -> bool,
        default: char,
    ) -> Grid<char>
    where
        char: Borrow<V>,
    {
        let mut grid: Grid<char> = Grid::new(default);

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if f(c) {
                    grid[Coordinate::new(x as isize, y as isize)] = c;
                }
            }
        }

        grid
    }

    /// Returns a reference to the value stored for the provided key.
    /// If the key is not in the `Grid` a reference to the default value is returned.
    /// Usually the `map[key]` method of retrieving keys is preferred over using `get` directly.
    /// This method accepts both references and owned values as the key.
    pub fn get<Q, QB: Borrow<Q>>(&self, key: QB) -> &V
    where
        Coordinate: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        self.map.get(key.borrow()).unwrap_or(&self.default)
    }

    /// Returns a mutable reference to the value stored for the provided key.
    /// If there is no value stored for the key the default value is first inserted for this
    /// key before returning the reference.
    /// Usually the `map[key] = new_val` is preferred over using `get_mut` directly.
    /// This method only accepts owned values as the key.
    pub fn get_mut(&mut self, key: Coordinate) -> &mut V {
        let default = &self.default;
        self.map.entry(key).or_insert_with(|| default.clone())
    }

    #[inline]
    pub fn has<Q>(&self, k: &Q) -> bool
    where
        Coordinate: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        self.map.contains_key(k)
    }
    #[inline]
    pub fn keys(&self) -> Keys<Coordinate, V> {
        self.map.keys()
    }
    #[inline]
    pub fn values(&self) -> Values<Coordinate, V> {
        self.map.values()
    }
    #[inline]
    pub fn values_mut(&mut self) -> ValuesMut<Coordinate, V> {
        self.map.values_mut()
    }
    #[inline]
    pub fn iter(&self) -> Iter<Coordinate, V> {
        self.map.iter()
    }
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<Coordinate, V> {
        self.map.iter_mut()
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn dimensions(&self) -> GridDimensions {
        let min_x = self.iter().map(|(c, _)| c.x).min().unwrap();
        let max_x = self.iter().map(|(c, _)| c.x).max().unwrap();
        let min_y = self.iter().map(|(c, _)| c.y).min().unwrap();
        let max_y = self.iter().map(|(c, _)| c.y).max().unwrap();

        GridDimensions {
            min_x,
            max_x,
            total_x: max_x - min_x,
            min_y,
            max_y,
            total_y: max_y - min_y,
        }
    }

    pub fn print(&self)
    where
        V: Copy + Debug,
    {
        let dimensions = self.dimensions();

        println!("printing grid (len={})", &self.len());

        for y in dimensions.min_y..=dimensions.max_y {
            for x in dimensions.min_x..=dimensions.max_x {
                let c = Coordinate::new(x, y);
                let data = format!("{:?}", self[c]);
                if data.starts_with('\'') && data.ends_with('\'') {
                    print!("{}", data.chars().rev().nth(1).unwrap());
                } else if data.starts_with('\"') && data.ends_with('\"') {
                    let mut data = data.chars().skip(1).collect_vec();
                    data.pop();
                    print!("{}", data.into_iter().collect::<String>());
                } else {
                    print!("{}", data.chars().next().unwrap());
                }
            }
            println!();
        }
        println!();
    }
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

    #[test]
    fn it_can_create_a_new_grid() {
        let expected: Grid<i8> = Grid::new(1i8);
        let actual: Grid<i8> = Grid {
            map: HashMap::new(),
            default: 1i8,
        };

        assert_eq!(expected, actual);
    }

    #[test]
    fn it_stores_coordinates() {
        let coordinates = [
            Coordinate::new(0, 1),
            Coordinate::new(0, -1),
            Coordinate::new(1, 0),
            Coordinate::new(-1, 0),
        ];
        let mut grid: Grid<i8> = Grid::new(-1i8);
        for coordinate in coordinates.iter() {
            grid[*coordinate] = 2;
        }

        assert_eq!(2, grid[Coordinate::new(0, 1)]);
        assert_eq!(2, grid[Coordinate::new(0, -1)]);
        assert_eq!(2, grid[Coordinate::new(1, 0)]);
        assert_eq!(2, grid[Coordinate::new(1, 0)]);
        assert_eq!(-1, grid[Coordinate::new(0, 0)]);
    }

    #[test]
    fn it_can_change_vector_values() {
        let coordinate_values = [
            (Coordinate::new(0, 1), 1),
            (Coordinate::new(0, -1), 2),
            (Coordinate::new(0, 1), 3),
            (Coordinate::new(0, 1), 4),
            (Coordinate::new(-1, 0), 5),
        ];

        let mut grid: Grid<Vec<i32>> = Grid::default();

        for &(coordinate, value) in coordinate_values.iter() {
            grid[coordinate].push(value);
        }

        assert_eq!(grid[Coordinate::new(0, -1)], vec![2]);
        assert_eq!(grid[Coordinate::new(0, 1)], vec![1, 3, 4]);
        assert_eq!(grid[Coordinate::new(0, 0)], Vec::<i32>::new());
    }

    #[test]
    fn it_can_create_a_grid_from_text() {
        let input = "..#..#";
        let grid = Grid::from_input(input, |c| c, ' ');

        assert_eq!(grid[Coordinate::new(0, 0)], '.');
        assert_eq!(grid[Coordinate::new(1, 0)], '.');
        assert_eq!(grid[Coordinate::new(2, 0)], '#');
        assert_eq!(grid[Coordinate::new(3, 0)], '.');
        assert_eq!(grid[Coordinate::new(4, 0)], '.');
        assert_eq!(grid[Coordinate::new(5, 0)], '#');
        assert_eq!(grid[Coordinate::new(6, 0)], ' ');
    }

    #[test]
    fn it_returns_correct_dimensions() {
        let mut grid = Grid::new('.');
        grid[Coordinate::new(-2, -2)] = '#';
        grid[Coordinate::new(2, 2)] = '#';

        let dimensions = grid.dimensions();

        assert_eq!(-2, dimensions.min_x);
        assert_eq!(2, dimensions.max_x);
        assert_eq!(4, dimensions.total_x);
        assert_eq!(-2, dimensions.min_y);
        assert_eq!(2, dimensions.max_y);
        assert_eq!(4, dimensions.total_y);
    }
}
