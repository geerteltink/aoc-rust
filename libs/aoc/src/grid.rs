use hashbrown::hash_map::*;
use hashbrown::HashMap;
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::ops::{Add, AddAssign, Index, IndexMut, Sub, SubAssign, Mul, Neg};

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

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Grid<K: Eq + Hash, V: Clone> {
    map: HashMap<K, V>,
    default: V,
}

impl<K: Eq + Hash, V: Default + Clone> Default for Grid<K, V> {
    /// The `default()` constructor creates an empty Grid with the default of `V`
    /// as the default for missing keys.
    /// This is desired default for most use cases, if your case requires a
    /// different default you should use the `new()` constructor.
    fn default() -> Grid<K, V> {
        Grid {
            map: HashMap::default(),
            default: V::default(),
        }
    }
}

impl<K: Eq + Hash, V: Default + Clone> From<HashMap<K, V>> for Grid<K, V> {
    /// If you already have a `HashMap` that you would like to convert to a
    /// `Grid` you can use the `into()` method on the `HashMap` or the
    /// `from()` constructor of `Grid`.
    /// The default value for missing keys will be `V::default()`,
    /// if this is not desired `Grid::new_with_map()` should be used.
    fn from(map: HashMap<K, V>) -> Grid<K, V> {
        Grid {
            map,
            default: V::default(),
        }
    }
}

impl<K: Eq + Hash, V: Clone> Into<HashMap<K, V>> for Grid<K, V> {
    /// The into method can be used to convert a `Grid` back into a
    /// `HashMap`.
    fn into(self) -> HashMap<K, V> {
        self.map
    }
}

/// Implements the `Index` trait so you can do `map[key]`.
/// Nonmutable indexing can be done both by passing a reference or an owned value as the key.
impl<'a, K: Eq + Hash, KB: Borrow<K>, V: Clone> Index<KB> for Grid<K, V> {
    type Output = V;

    fn index(&self, index: KB) -> &V {
        self.get(index)
    }
}

/// Implements the `IndexMut` trait so you can do `map[key] = val`.
/// Mutably indexing can only be done when passing an owned value as the key.
impl<K: Eq + Hash, V: Clone> IndexMut<K> for Grid<K, V> {
    #[inline]
    fn index_mut(&mut self, index: K) -> &mut V {
        self.get_mut(index)
    }
}

impl<K: Eq + Hash, V: Clone> Grid<K, V> {
    /// Creates an empty `Grid` with `default` as the default for missing keys.
    /// When the provided `default` is equivalent to `V::default()` it is preferred to use
    /// `Grid::default()` instead.
    pub fn new(default: V) -> Grid<K, V> {
        Grid {
            map: HashMap::new(),
            default,
        }
    }

    pub fn from_input(
        input: &str,
        mut f: impl FnMut(char) -> V,
        default: V,
    ) -> Grid<Coordinate, V>
    where
        Coordinate: Borrow<K>,
    {
        let mut grid: Grid<Coordinate, V> = Grid::new(default);

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
    ) -> Grid<Coordinate, char>
    where
        Coordinate: Borrow<K>,
        char: Borrow<V>,
    {
        let mut grid: Grid<Coordinate, char> = Grid::new(default);

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
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        self.map.get(key.borrow()).unwrap_or(&self.default)
    }

    /// Returns a mutable reference to the value stored for the provided key.
    /// If there is no value stored for the key the default value is first inserted for this
    /// key before returning the reference.
    /// Usually the `map[key] = new_val` is preferred over using `get_mut` directly.
    /// This method only accepts owned values as the key.
    pub fn get_mut(&mut self, key: K) -> &mut V {
        let default = &self.default;
        self.map.entry(key).or_insert_with(|| default.clone())
    }

    #[inline]
    pub fn has<Q>(&self, k: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        self.map.contains_key(k)
    }
    #[inline]
    pub fn keys(&self) -> Keys<K, V> {
        self.map.keys()
    }
    #[inline]
    pub fn values(&self) -> Values<K, V> {
        self.map.values()
    }
    #[inline]
    pub fn values_mut(&mut self) -> ValuesMut<K, V> {
        self.map.values_mut()
    }
    #[inline]
    pub fn iter(&self) -> Iter<K, V> {
        self.map.iter()
    }
    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<K, V> {
        self.map.iter_mut()
    }
    #[inline]
    pub fn len(&self) -> usize {
        self.map.len()
    }
}

/*
//pub type Grid<T> = Grid<Coordinate, T>;

/// Usage:
///   let arena = create_grid_from_input(&input, |c| c, '#');
pub fn create_grid_from_input<T: Clone>(
    s: &str,
    mut f: impl FnMut(char) -> T,
    default: T,
) -> Grid<T> {
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
*/

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
        let expected: Grid<Coordinate, i8> = Grid::new(1i8);
        let actual: Grid<Coordinate, i8> = Grid {
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
        let mut grid: Grid<Coordinate, i8> = Grid::new(-1i8);
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

        let mut grid: Grid<Coordinate, Vec<i32>> = Grid::default();

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
}
