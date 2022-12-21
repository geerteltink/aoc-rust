use aoc::*;

static DAY: &'static str = "17";

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

fn part_one(input: &String) -> usize {
    // The jet directions keep repeating
    let mut directions = input.trim().chars().cycle();

    let rocks = [
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
        vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)],
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(0, 0), (0, 1), (1, 0), (1, 1)],
    ];

    let mut map = vec![[false; 7]; 0];
    let mut current_rock = 0;
    let mut current_pos = (3isize, 2isize);
    let mut rocks_dropped = 0u16;

    while rocks_dropped < 2022 {
        // Get next hot gas jet direction
        let direction_x = match directions.next() {
            Some('<') => current_pos.1 - 1,
            Some('>') => current_pos.1 + 1,
            _ => panic!(),
        };

        // Move rock left or right
        if direction_x >= 0
            && rocks[current_rock].iter().all(|&(y, x)| {
                let (abs_y, abs_x) = (current_pos.0 as usize + y, direction_x as usize + x);
                abs_x < 7 && (abs_y >= map.len() || !map[abs_y][abs_x])
            })
        {
            current_pos.1 = direction_x;
        }

        // Drop rock
        let direction_y = current_pos.0 - 1;
        if direction_y < 0
            || rocks[current_rock].iter().any(|&(y, x)| {
                let (abs_y, abs_x) = (direction_y as usize + y, current_pos.1 as usize + x);
                abs_y < map.len() && map[abs_y][abs_x]
            })
        {
            // Update map
            for &(y, x) in &rocks[current_rock] {
                let (abs_y, abs_x) = (current_pos.0 as usize + y, current_pos.1 as usize + x);
                while abs_y >= map.len() {
                    map.push([false; 7]);
                }
                map[abs_y][abs_x] = true;
            }

            // Update next position for the new rock
            current_pos = (map.len() as isize + 3, 2);

            // Get the next rock
            current_rock = (current_rock + 1) % rocks.len();

            // Update total dropped rocks
            rocks_dropped += 1;
        } else {
            current_pos.0 = direction_y;
        }
    }

    return map.len();
}

fn part_two(input: &String) -> usize {
    // The jet directions keep repeating
    let mut directions = input.trim().chars().cycle();

    let rocks = [
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
        vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)],
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(0, 0), (0, 1), (1, 0), (1, 1)],
    ];

    let mut map = vec![[false; 7]; 0];
    let mut current_rock = 0;
    let mut current_pos = (3isize, 2isize);
    let mut rocks_dropped = 0u64;

    // TODO: FIX THIS
    while rocks_dropped < 1_000_000_000_000 {
        // Get next hot gas jet direction
        let direction_x = match directions.next() {
            Some('<') => current_pos.1 - 1,
            Some('>') => current_pos.1 + 1,
            _ => panic!(),
        };

        // Move rock left or right
        if direction_x >= 0
            && rocks[current_rock].iter().all(|&(y, x)| {
                let (abs_y, abs_x) = (current_pos.0 as usize + y, direction_x as usize + x);
                abs_x < 7 && (abs_y >= map.len() || !map[abs_y][abs_x])
            })
        {
            current_pos.1 = direction_x;
        }

        // Drop rock
        let direction_y = current_pos.0 - 1;
        if direction_y < 0
            || rocks[current_rock].iter().any(|&(y, x)| {
                let (abs_y, abs_x) = (direction_y as usize + y, current_pos.1 as usize + x);
                abs_y < map.len() && map[abs_y][abs_x]
            })
        {
            // Update map
            for &(y, x) in &rocks[current_rock] {
                let (abs_y, abs_x) = (current_pos.0 as usize + y, current_pos.1 as usize + x);
                while abs_y >= map.len() {
                    map.push([false; 7]);
                }
                map[abs_y][abs_x] = true;
            }

            // Update next position for the new rock
            current_pos = (map.len() as isize + 3, 2);

            // Get the next rock
            current_rock = (current_rock + 1) % rocks.len();

            // Update total dropped rocks
            rocks_dropped += 1;
        } else {
            current_pos.0 = direction_y;
        }
    }

    return map.len();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_the_test_answer_for_part_one() {
        let input = load_input("./fixtures/input_test.txt");
        assert_eq!(3068, part_one(&input));
    }

    #[ignore]
    #[test]
    fn it_returns_the_test_answer_for_part_two() {
        let input = load_input("./fixtures/input_test.txt");
        assert_eq!(1514285714288, part_two(&input));
    }

    #[test]
    fn it_returns_the_answer_for_part_one() {
        let input = load_input("./fixtures/input.txt");
        assert_eq!(3069, part_one(&input));
    }

    #[ignore]
    #[test]
    fn it_returns_the_answer_for_part_two() {
        let input = load_input("./fixtures/input.txt");
        assert_eq!(0, part_two(&input));
    }
}
