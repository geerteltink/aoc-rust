use aoc::grid::*;
use aoc::*;

static DAY: &'static str = "22";

const OUT_OF_BOUNDS: char = ' ';
const WALL: char = '#';

#[derive(Debug)]
enum Instruction {
    Move(usize),
    TurnRight,
    TurnLeft,
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Player {
    loc: Coordinate,
    dir: Direction,
}

impl Player {
    fn turn_left(&mut self) {
        match self.dir {
            Direction::Up => self.dir = Direction::Left,
            Direction::Left => self.dir = Direction::Down,
            Direction::Down => self.dir = Direction::Right,
            Direction::Right => self.dir = Direction::Up,
        }
    }

    fn turn_right(&mut self) {
        match self.dir {
            Direction::Up => self.dir = Direction::Right,
            Direction::Right => self.dir = Direction::Down,
            Direction::Down => self.dir = Direction::Left,
            Direction::Left => self.dir = Direction::Up,
        }
    }

    fn walk(&mut self, x: usize, board: &Grid<char>) {
        for _ in 0..x {
            let mut new_loc = self.loc;
            match self.dir {
                Direction::Up => new_loc += Coordinate::new(0, -1),
                Direction::Down => new_loc += Coordinate::new(0, 1),
                Direction::Right => new_loc += Coordinate::new(1, 0),
                Direction::Left => new_loc += Coordinate::new(-1, 0),
            }

            let board_loc = board[new_loc];
            if board_loc == WALL {
                //debug_println!("Hit wall {:?}", &new_loc);
                // do nothing if a wall is hit
                return;
            }

            if board_loc == OUT_OF_BOUNDS {
                // continue on the other side of the board
                match self.dir {
                    Direction::Up => {
                        new_loc = board
                            .iter()
                            .filter(|(loc, value)| loc.x == new_loc.x && value != &&OUT_OF_BOUNDS)
                            .max_by(|a, b| a.0.y.cmp(&b.0.y))
                            .unwrap()
                            .0
                            .to_owned()
                    }
                    Direction::Down => {
                        new_loc = board
                            .iter()
                            .filter(|(loc, value)| loc.x == new_loc.x && value != &&OUT_OF_BOUNDS)
                            .min_by(|a, b| a.0.y.cmp(&b.0.y))
                            .unwrap()
                            .0
                            .to_owned()
                    }
                    Direction::Right => {
                        new_loc = board
                            .iter()
                            .filter(|(loc, value)| loc.y == new_loc.y && value != &&OUT_OF_BOUNDS)
                            .min_by(|a, b| a.0.x.cmp(&b.0.x))
                            .unwrap()
                            .0
                            .to_owned()
                    }
                    Direction::Left => {
                        new_loc = board
                            .iter()
                            .filter(|(loc, value)| loc.y == new_loc.y && value != &&OUT_OF_BOUNDS)
                            .max_by(|a, b| a.0.x.cmp(&b.0.x))
                            .unwrap()
                            .0
                            .to_owned()
                    }
                }

                let board_loc = board[new_loc];
                if board_loc == WALL {
                    //debug_println!("Hit wall on the other side{:?}", &new_loc);
                    // do nothing if a wall is hit
                    return;
                }

                //debug_println!("Out of bounds {:?}", &new_loc);
            }

            //debug_println!("Moved {:?} from {:?} -> {:?}", &self.dir, old_loc, &new_loc);

            self.loc = new_loc;
        }
    }

    fn walk_cube(&mut self, x: usize, board: &Grid<char>) {
        let mut cube_width = 4;
        if board.len() > 160 {
            cube_width = 50;
        }

        for _ in 0..x {
            let cur_loc = self.loc;
            let mut new_loc = self.loc;
            let mut new_dir = self.dir;
            match self.dir {
                Direction::Up => new_loc += Coordinate::new(0, -1),
                Direction::Down => new_loc += Coordinate::new(0, 1),
                Direction::Right => new_loc += Coordinate::new(1, 0),
                Direction::Left => new_loc += Coordinate::new(-1, 0),
            }

            let board_loc = board[new_loc];
            if board_loc == WALL {
                //debug_println!("Hit wall {:?}", &new_loc);
                // do nothing if a wall is hit
                return;
            }

            if board_loc == OUT_OF_BOUNDS {
                let block = cur_loc / cube_width;
                let offset = cur_loc - block * cube_width;

                debug_println!(
                    "\n\nOut of bounds from quadrant ({:?},{:?}) - {:?}",
                    cur_loc.x / cube_width,
                    cur_loc.y / cube_width,
                    self.dir
                );

                debug_println!("cur_loc {}", &cur_loc);
                debug_println!("offset  {}", &offset);

                if cube_width == 4 {
                    // test input
                    match (cur_loc.x / cube_width, cur_loc.y / cube_width, &self.dir) {
                        (2, 1, Direction::Right) => {
                            new_loc = Coordinate::new(3, 2) * cube_width;
                            new_loc += Coordinate::new(offset.x - 1, 0);
                            new_dir = Direction::Down;
                        }
                        (2, 2, Direction::Down) => {
                            new_loc = Coordinate::new(0, 1) * cube_width;
                            new_loc += Coordinate::new(cube_width - offset.x - 1, cube_width - 1);
                            new_dir = Direction::Up;
                        }
                        (1, 1, Direction::Up) => {
                            new_loc = Coordinate::new(2, 0) * cube_width;
                            new_loc += Coordinate::new(0, offset.x);
                            new_dir = Direction::Right;
                        }
                        _ => unreachable!(),
                    }
                } else {
                    // challenge input
                    match (cur_loc.x / cube_width, cur_loc.y / cube_width, &self.dir) {
                        // Out of bounds from quadrant (0,2) - Up
                        (0, 2, Direction::Up) => {
                            new_loc = Coordinate::new(1, 1) * cube_width;
                            new_loc += Coordinate::new(0, offset.x);
                            new_dir = Direction::Right;
                        }
                        // Out of bounds from quadrant (0,2) - Left
                        (0, 2, Direction::Left) => {
                            new_loc = Coordinate::new(1, 0) * cube_width;
                            new_loc += Coordinate::new(0, cube_width - 1 - offset.y);
                            new_dir = Direction::Right;
                        }
                        // Out of bounds from quadrant (0,3) - Right
                        (0, 3, Direction::Right) => {
                            new_loc = Coordinate::new(1, 2) * cube_width;
                            new_loc += Coordinate::new(offset.y, cube_width - 1);
                            new_dir = Direction::Up;
                        }
                        // Out of bounds from quadrant (0,3) - Down
                        (0, 3, Direction::Down) => {
                            new_loc = Coordinate::new(2, 0) * cube_width;
                            new_loc += Coordinate::new(offset.x, 0);
                            new_dir = Direction::Down;
                        }
                        // Out of bounds from quadrant (0,3) - Left
                        (0, 3, Direction::Left) => {
                            new_loc = Coordinate::new(1, 0) * cube_width;
                            new_loc += Coordinate::new(offset.y, 0);
                            new_dir = Direction::Down;
                        }
                        // Out of bounds from quadrant (1,0) - Up
                        (1, 0, Direction::Up) => {
                            new_loc = Coordinate::new(0, 3) * cube_width;
                            new_loc += Coordinate::new(0, offset.x);
                            new_dir = Direction::Right;
                        }
                        // Out of bounds from quadrant (1,0) - Left
                        (1, 0, Direction::Left) => {
                            new_loc = Coordinate::new(0, 2) * cube_width;
                            new_loc += Coordinate::new(0, cube_width - 1 - offset.y);
                            new_dir = Direction::Right;
                        }
                        // Out of bounds from quadrant (1,1) - Right
                        (1, 1, Direction::Right) => {
                            new_loc = Coordinate::new(2, 0) * cube_width;
                            new_loc += Coordinate::new(offset.y, cube_width - 1);
                            new_dir = Direction::Up;
                        }
                        // Out of bounds from quadrant (1,1) - Left
                        (1, 1, Direction::Left) => {
                            new_loc = Coordinate::new(0, 2) * cube_width;
                            new_loc += Coordinate::new(offset.y, 0);
                            new_dir = Direction::Down;
                        }
                        // Out of bounds from quadrant (1,2) - Right
                        (1, 2, Direction::Right) => {
                            new_loc = Coordinate::new(2, 0) * cube_width;
                            new_loc += Coordinate::new(cube_width - 1, cube_width - 1 - offset.y);
                            new_dir = Direction::Left;
                        }
                        // Out of bounds from quadrant (1,2) - Down
                        (1, 2, Direction::Down) => {
                            new_loc = Coordinate::new(0, 3) * cube_width;
                            new_loc += Coordinate::new(cube_width - 1, offset.x);
                            new_dir = Direction::Left;
                        }
                        // Out of bounds from quadrant (2,0) - Up
                        (2, 0, Direction::Up) => {
                            new_loc = Coordinate::new(0, 3) * cube_width;
                            new_loc += Coordinate::new(offset.x, cube_width - 1);
                            new_dir = Direction::Up;
                        }
                        // Out of bounds from quadrant (2,0) - Right
                        (2, 0, Direction::Right) => {
                            new_loc = Coordinate::new(1, 2) * cube_width;
                            new_loc += Coordinate::new(cube_width - 1, cube_width - 1 - offset.y);
                            new_dir = Direction::Left;
                        }
                        // Out of bounds from quadrant (2,0) - Down
                        (2, 0, Direction::Down) => {
                            new_loc = Coordinate::new(1, 1) * cube_width;
                            new_loc += Coordinate::new(cube_width - 1, offset.x);
                            new_dir = Direction::Left;
                        }
                        _ => unreachable!(),
                    }
                }

                debug_println!("new_loc {} -> {:?}", &new_loc, &new_dir);

                let board_loc = board[new_loc];
                if board_loc == WALL {
                    //debug_println!("Hit wall on the other side{:?}", &new_loc);
                    // do nothing if a wall is hit
                    return;
                }

                //debug_println!("Out of bounds {:?}", &new_loc);
            }

            //debug_println!("Moved {:?} from {:?} -> {:?}", &self.dir, &cur_loc, &new_loc);

            self.loc = new_loc;
            self.dir = new_dir;
        }
    }
}

fn main() {
    let input = load_input(format!("./2022/day_{DAY}/fixtures/input.txt"));

    let start = Instant::now();
    let result1 = part_one(&input);
    let end = Instant::now();
    println!("Elapsed time: {:?}", end - start);
    println!("Answer day {DAY} part one: {result1} in {:?}", end - start);

    let start = Instant::now();
    let result2 = part_two(&input);
    let end = Instant::now();
    println!("Answer day {DAY} part two: {result2} in {:?}", end - start);
}

fn part_one(input: &String) -> isize {
    let (board_rows, directions) = input.split_once("\n\n").unwrap();
    let board = Grid::from_input(board_rows, |c| c, OUT_OF_BOUNDS);

    //board.print();

    // convert directions to instructions
    let mut instructions = Vec::new();
    let mut num = 0;
    for c in directions.trim().chars() {
        if c.is_ascii_digit() {
            num = num * 10 + c.to_digit(10).unwrap();
        } else {
            if num != 0 {
                instructions.push(Instruction::Move(num as usize));
                num = 0;
            }
            match c {
                'R' => instructions.push(Instruction::TurnRight),
                'L' => instructions.push(Instruction::TurnLeft),
                _ => unreachable!(),
            }
        }
    }
    // add a possible left over movement to the instructions
    if num != 0 {
        instructions.push(Instruction::Move(num as usize));
    }

    // find player start position
    let player_start = board
        .iter()
        .filter(|(loc, value)| loc.y == 0 && value != &&OUT_OF_BOUNDS)
        .min_by(|a, b| a.0.x.cmp(&b.0.x))
        .unwrap()
        .0
        .to_owned();

    let mut player = Player {
        loc: player_start,
        dir: Direction::Right,
    };

    // I like to move it, move it
    for instruction in instructions {
        match instruction {
            Instruction::TurnLeft => player.turn_left(),
            Instruction::TurnRight => player.turn_right(),
            Instruction::Move(x) => player.walk(x, &board),
        }
    }

    // Facing is 0 for right (>), 1 for down (v), 2 for left (<), and 3 for up (^).
    let mut facing = 0;
    match player.dir {
        Direction::Right => facing = facing + 0,
        Direction::Down => facing = facing + 1,
        Direction::Left => facing = facing + 2,
        Direction::Up => facing = facing + 3,
    }

    // The final password is the sum of 1000 times the row, 4 times the column, and the facing.
    let answer = (player.loc.y + 1) * 1000 + (player.loc.x + 1) * 4;

    return answer + facing;
}

fn part_two(input: &String) -> isize {
    let (board_rows, directions) = input.split_once("\n\n").unwrap();
    let board = Grid::from_input(board_rows, |c| c, OUT_OF_BOUNDS);

    //board.print();

    // convert directions to instructions
    let mut instructions = Vec::new();
    let mut num = 0;
    for c in directions.trim().chars() {
        if c.is_ascii_digit() {
            num = num * 10 + c.to_digit(10).unwrap();
        } else {
            if num != 0 {
                instructions.push(Instruction::Move(num as usize));
                num = 0;
            }
            match c {
                'R' => instructions.push(Instruction::TurnRight),
                'L' => instructions.push(Instruction::TurnLeft),
                _ => unreachable!(),
            }
        }
    }
    // add a possible left over movement to the instructions
    if num != 0 {
        instructions.push(Instruction::Move(num as usize));
    }

    // find player start position
    let player_start = board
        .iter()
        .filter(|(loc, value)| loc.y == 0 && value != &&OUT_OF_BOUNDS)
        .min_by(|a, b| a.0.x.cmp(&b.0.x))
        .unwrap()
        .0
        .to_owned();

    let mut player = Player {
        loc: player_start,
        dir: Direction::Right,
    };

    // I like to move it, move it
    for instruction in instructions {
        match instruction {
            Instruction::TurnLeft => player.turn_left(),
            Instruction::TurnRight => player.turn_right(),
            Instruction::Move(x) => player.walk_cube(x, &board),
        }
    }

    debug_println!("{:?}", &player);

    // Facing is 0 for right (>), 1 for down (v), 2 for left (<), and 3 for up (^).
    let mut facing = 0;
    match player.dir {
        Direction::Right => facing = facing + 0,
        Direction::Down => facing = facing + 1,
        Direction::Left => facing = facing + 2,
        Direction::Up => facing = facing + 3,
    }

    // The final password is the sum of 1000 times the row, 4 times the column, and the facing.
    let answer = (player.loc.y + 1) * 1000 + (player.loc.x + 1) * 4;

    return answer + facing;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_the_test_answer_for_part_one() {
        let input = load_input("./fixtures/input_test.txt");
        assert_eq!(6032, part_one(&input));
    }

    #[test]
    fn it_returns_the_answer_for_part_one() {
        let input = load_input("./fixtures/input.txt");
        assert_eq!(56372, part_one(&input));
    }

    #[test]
    fn it_returns_the_test_answer_for_part_two() {
        let input = load_input("./fixtures/input_test.txt");
        assert_eq!(5031, part_two(&input));
    }

    #[test]
    fn it_returns_the_answer_for_part_two() {
        let input = load_input("./fixtures/input.txt");
        assert_eq!(197047, part_two(&input));
    }
}
