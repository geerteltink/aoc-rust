use aoc::*;
use aoc::arena::*;

static DAY: &'static str = "08";

fn main() {
    let input = load_input(format!("./2022/day_{DAY}/fixtures/input.txt"));

    let result1 = part_one(&input);
    println!("Answer day {DAY} part one: {result1}");

    let result2 = part_two(&input);
    println!("Answer day {DAY} part one: {result2}");
}

fn part_one(input: &String) -> i32 {
    let mut grid: DefaultHashMap<Loc, i8> = DefaultHashMap::new(-1i8);
    
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            grid[Loc::new(x as i64,y as i64)] = c as i8;
        }
    }

    let mut total_visible = 0;
    for (&pos, &height) in grid.iter() {
        let mut visible_from_outside = false;
        for direction in Loc::NEIGHBORS {
            let visible = pos
                .walk(direction)
                .test(|p| grid[p] == -1, |p| grid[p] >= height);

            if visible {
                visible_from_outside = true;
            }
        }

        if visible_from_outside {
            total_visible += 1;
        }
    }

    //println!("{:#?}", grid);
    //println!("{:?}", total_visible);

    return total_visible;
}

fn part_two(input: &String) -> usize {
    let mut grid: DefaultHashMap<Loc, i8> = DefaultHashMap::new(-1i8);
    
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.trim().chars().enumerate() {
            grid[Loc::new(x as i64,y as i64)] = c as i8;
        }
    }
    
    let mut max = 0;
    for (&pos, &height) in grid.iter() {
        let mut scenic_score = 1;
        for direction in Loc::NEIGHBORS {
            let visible = pos
                .walk(direction)
                .test(|p| grid[p] == -1, |p| grid[p] >= height);

            let mut distance = pos
                .walk(direction)
                .take_while(|p| grid[p] < height && grid[p] != -1)
                .count();

            if !visible {
                distance += 1;
            }

            scenic_score *= distance;
        }

        max = scenic_score.max(max);
    }

    //println!("{:#?}", grid);
    //println!("{:?}", total_visible);

    return max;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_the_answer_for_part_one() {
        let input = std::fs::read_to_string("./fixtures/input_test.txt").unwrap();
        assert_eq!(21, part_one(&input));
    }

    #[test]
    fn it_returns_the_answer_for_part_two() {
        let input = std::fs::read_to_string("./fixtures/input_test.txt").unwrap();
        assert_eq!(8, part_two(&input));
    }
}
