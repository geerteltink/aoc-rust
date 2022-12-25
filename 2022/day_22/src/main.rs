use aoc::arena::*;
use aoc::*;

static DAY: &'static str = "22";

const OUT_OF_BOUNDS: i8 = b' ' as i8;
const WALL: i8 = b'#' as i8;

#[derive(Debug)]
enum Instruction {
    Move(usize),
    TurnRight,
    TurnLeft,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Player {
    loc: Loc,
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

    fn walk(&mut self, x: usize, board: &DefaultHashMap<Loc, i8>) {
        for _ in 0..x {
            let old_loc = self.loc;
            let mut new_loc = self.loc;
            let mut movement = Loc::new(0, 0);
            match self.dir {
                Direction::Up => movement = Loc::new(0, -1),
                Direction::Down => movement = Loc::new(0, 1),
                Direction::Right => movement = Loc::new(1, 0),
                Direction::Left => movement = Loc::new(-1, 0),
            }

            new_loc += movement;

            let board_loc = board.get(new_loc);
            if board_loc == &WALL {
                println!("Hit wall {:?}", &new_loc);
                // do nothing if a wall is hit
                return;
            }

            if board_loc == &OUT_OF_BOUNDS {
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

                let board_loc = board.get(new_loc);
                if board_loc == &WALL {
                    println!("Hit wall on the other side{:?}", &new_loc);
                    // do nothing if a wall is hit
                    return;
                }

                println!("Out of bounds {:?}", &new_loc);
            }

            println!("Moved {:?} from {:?} -> {:?}", &self.dir, old_loc, &new_loc);

            self.loc = new_loc;
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

fn part_one(input: &String) -> i64 {
    let mut board: DefaultHashMap<Loc, i8> = DefaultHashMap::new(OUT_OF_BOUNDS);

    // add all non empty spaces to the board
    let (board_rows, directions) = input.split_once("\n\n").unwrap();
    for (y, board_row) in board_rows.lines().enumerate() {
        for (x, c) in board_row.chars().enumerate() {
            let c = c as i8;
            if c != OUT_OF_BOUNDS {
                board[Loc::new((x + 1) as i64, (y + 1) as i64)] = c;
            }
        }
    }

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
        .filter(|(loc, value)| loc.y == 1 && value != &&OUT_OF_BOUNDS)
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

    // The final password is the sum of 1000 times the row, 4 times the column, and the facing.
    let answer = player.loc.y * 1000 + player.loc.x * 4;

    return answer;
}

fn part_two(_input: &String) -> isize {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    // Start 10   R 5    L 5   R 10  L 4   R 5   L 5
    // R 9.1 11.1 D 11.6 R 4.6 D 4.8 R 8.8 D 8.6 R 8.6
    #[test]
    fn it_returns_the_test_answer_for_part_one() {
        let input = load_input("./fixtures/input_test.txt");
        assert_eq!(6032, part_one(&input));
    }

    #[ignore]
    #[test]
    fn it_returns_the_test_answer_for_part_two() {
        let input = load_input("./fixtures/input_test.txt");
        assert_eq!(5031, part_two(&input));
    }

    #[test]
    fn it_returns_the_answer_for_part_one() {
        let input = load_input("./fixtures/input.txt");
        assert_eq!(56372, part_one(&input));
    }

    #[ignore]
    #[test]
    fn it_returns_the_answer_for_part_two() {
        let input = load_input("./fixtures/input.txt");
        assert_eq!(0, part_two(&input));
    }
}
