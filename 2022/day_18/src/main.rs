use aoc::*;
use aoc::arena3d::*;

static DAY: &'static str = "18";

fn main() {
    let input = load_input(format!("./2022/day_{DAY}/fixtures/input.txt"));

    let result1 = part_one(&input);
    println!("Answer day {DAY} part one: {result1}");

    let result2 = part_two(&input);
    println!("Answer day {DAY} part two: {result2}");
}

fn part_one(input: &String) -> isize {
    let mut arena = Arena3D::new();

    for line in input.lines() {
        let [x, y, z] = extract_numbers(line);
        arena.insert(Loc3D::new(x, y, z));
    }

    let mut count = 0;

    // Count up all the sides that aren't connected to another cube
    for cube in arena.map.clone() {
        for neighbor in Loc3D::NEIGHBORS {
            let side = cube + neighbor;
            if !arena.map.contains(&side) {
                count += 1;
            }
        }
    }

    return count;
}

fn part_two(input: &String) -> isize {
    let mut arena = Arena3D::new();

    for line in input.lines() {
        let [x, y, z] = extract_numbers(line);
        arena.insert(Loc3D::new(x, y, z));
    }

    let mut visited: HashSet<_> = Default::default();
    let mut stack: Vec<_> = Default::default();
    let mut count = 0;
    
    let start = *arena.map.iter().min_by_key(|x| x.x).unwrap() + Loc3D::new(-1, 0, 0);
    visited.insert(start);
    stack.push(start);

    while let Some(next) = stack.pop() {
        for direction in Loc3D::NEIGHBORS.iter() {
            let new_point = next + *direction;

            if arena.map.contains(&new_point) {
                count += 1;
            } else if arena.is_in_bounds(&new_point)
                && !visited.contains(&new_point)
                && arena.is_adjacent(&new_point)
            {
                visited.insert(new_point);
                stack.push(new_point);
            }
        }
    }

    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_the_answer_for_part_one() {
        let input = load_input("./fixtures/input_test.txt");
        assert_eq!(64, part_one(&input));
    }

    #[test]
    fn it_returns_the_answer_for_part_two() {
        let input = load_input("./fixtures/input_test.txt");
        assert_eq!(58, part_two(&input));
    }
}
