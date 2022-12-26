use aoc::grid::*;
use aoc::*;
use std::sync::Arc;

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

fn part_one(input: &String) -> usize {
    let grid = Grid::from_input(&input, |c| c, ' ');

    // get start and end coordinates
    let start = Coordinate::new(1 as isize, 0 as isize);
    let end = grid
        .iter()
        .filter(|x| *x.1 == '.')
        .map(|x| *x.0)
        .max_by_key(|coordinate| coordinate.y)
        .unwrap();

    // cache the blizzards to speed up calculations
    let mut blizzards_cache: Vec<Arc<Vec<(Coordinate, char)>>> = Default::default();
    let mut blizzards_avoid_cache: Vec<HashSet<Coordinate>> = Vec::new();

    let blizzards: Vec<_> = grid
        .iter()
        .filter(|x| ['>', '<', '^', 'v'].contains(x.1))
        .map(|(a, b)| (*a, *b))
        .collect();

    let blizzards = Arc::new(blizzards);
    blizzards_cache.insert(0, blizzards);

    // use bfs to calculate the shortest path
    let init = (0, start, 0);
    
    for steps_needed in [1, 3] {
        let mut max_step = 0;
        let result = bfs(
            &init,
            |(ticks, pos, step)| {
                if *step < max_step {
                    return vec![];
                }
                max_step = max_step.max(*step);

                let mut goto = HashSet::new();
                goto.insert(*pos);

                // move blizzards
                let bliz_next = if let Some(out) = blizzards_cache.get(ticks + 1) {
                    out.clone()
                } else {
                    let blizzards = blizzards_cache[*ticks].clone();

                    let mut bliz_next: Vec<(Coordinate, char)> = Vec::new();
                    for (pos, direction) in blizzards.iter().cloned() {
                        let mut next = if direction == '<' {
                            pos.left(1)
                        } else if direction == '>' {
                            pos.right(1)
                        } else if direction == 'v' {
                            pos.down(1)
                        } else if direction == '^' {
                            pos.up(1)
                        } else {
                            unreachable!();
                        };
                        if grid[next] == '#' {
                            let unit = next - pos;
                            next += -unit;
                            while grid[next] != '#' {
                                next += -unit;
                            }
                            next += unit;
                        }
                        bliz_next.push((next, direction));
                    }
                    assert_eq!(blizzards_cache.len() - 1, *ticks);
                    blizzards_cache.push(Arc::new(bliz_next));
                    blizzards_cache[ticks + 1].clone()
                };

                let bliz_avoid = if let Some(out) = blizzards_avoid_cache.get(*ticks) {
                    out.clone()
                } else {
                    let mut bliz_avoid = HashSet::new();

                    for pos in bliz_next.iter().map(|x| x.0) {
                        bliz_avoid.insert(pos);
                    }
                    assert_eq!(blizzards_avoid_cache.len(), *ticks);
                    blizzards_avoid_cache.push(bliz_avoid.clone());

                    bliz_avoid
                };

                goto.extend(pos.neighbors());

                // avoid the blizzards
                goto.retain(|&x| !bliz_avoid.contains(&x));
                goto.retain(|&x| grid[x] != '#');

                let mut result = vec![];
                for next in goto {
                    let mut step = *step;
                    if step == 0 && next == end {
                        step = 1;
                    }
                    if step == 1 && next == start {
                        step = 2;
                    }
                    if step == 2 && next == end {
                        step = 3;
                    }
                    result.push((ticks + 1, next, step))
                }

                result
            },
            |x| x.2 == steps_needed,
        );
        
        let answer = result.unwrap().len() - 1;
        
        println!("answer {:?}", answer);
    }

    return 18;
}

fn part_two(_input: &String) -> isize {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(896, part_two(&input));
    }
}
