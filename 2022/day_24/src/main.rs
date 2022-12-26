use aoc::grid::*;
use aoc::*;

static DAY: &'static str = "24";

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
    let grid = create_grid_from_input(&input, |c| c, '#');

    let start = Coordinate::new(1 as isize, 0 as isize);
    let end = grid
        .iter()
        .filter(|x| *x.1 == '.')
        .map(|x| *x.0)
        .max_by_key(|coordinate| coordinate.y)
        .unwrap();
    
    dbg!(&grid, &start, &end);
        
    return 0;
}

fn part_two(_input: &String) -> isize {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn it_returns_the_test_answer_for_part_one() {
        let input = load_input("./fixtures/input_test.txt");
        assert_eq!(18, part_one(&input));
    }

    #[ignore]
    #[test]
    fn it_returns_the_answer_for_part_one() {
        let input = load_input("./fixtures/input.txt");
        assert_eq!(314, part_one(&input));
    }

    #[ignore]
    #[test]
    fn it_returns_the_test_answer_for_part_two() {
        let input = load_input("./fixtures/input_test.txt");
        assert_eq!(54, part_two(&input));
    }

    #[ignore]
    #[test]
    fn it_returns_the_answer_for_part_two() {
        let input = load_input("./fixtures/input.txt");
        assert_eq!(0, part_two(&input));
    }
}
