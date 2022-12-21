use aoc::*;

static DAY: &'static str = "19";

#[derive(Debug)]
struct Blueprint {
    id: u32,
    ore_robot_ore_cost: u32,
    clay_robot_ore_cost: u32,
    obsidian_robot_ore_cost: u32,
    obsidian_robot_clay_cost: u32,
    geode_robot_ore_cost: u32,
    geode_robot_obsidian_cost: u32,
}

#[derive(Default, Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct State {
    ore: u32,
    ore_robots: u32,
    clay: u32,
    clay_robots: u32,
    obsidian: u32,
    obsidian_robots: u32,
    geodes: u32,
    geode_robots: u32,
}

impl State {
    fn new() -> Self {
        Self {
            ore: 0,
            ore_robots: 1,
            clay: 0,
            clay_robots: 0,
            obsidian: 0,
            obsidian_robots: 0,
            geodes: 0,
            geode_robots: 0,
        }
    }
    
    fn collect_resources(&self) -> Self {
        Self {
            ore: self.ore + self.ore_robots,
            clay: self.clay + self.clay_robots,
            obsidian: self.obsidian + self.obsidian_robots,
            geodes: self.geodes + self.geode_robots,
            ..*self
        }
    }
}

// https://en.wikipedia.org/wiki/Breadth-first_search
// 
// Declare a queue and insert the starting vertex.
// Initialize a visited array and mark the starting vertex as visited.
// Follow the below process till the queue becomes empty:
//   - Remove the first vertex of the queue.
//   - Mark that vertex as visited.
//   - Insert all the unvisited neighbours of the vertex into the queue.
// 
fn bfs(blueprint: &Blueprint, time: u32) -> u32 {
    // Declare a queue and insert the starting vertex.
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();

    let max_ore_cost = *[
        blueprint.ore_robot_ore_cost, 
        blueprint.clay_robot_ore_cost, 
        blueprint.obsidian_robot_ore_cost, 
        blueprint.geode_robot_ore_cost
    ].iter().max().unwrap();

    // Initialize a visited array and mark the starting vertex as visited.
    queue.push_back((0, State::new()));

    let mut answer = 0;
    
    // Follow the below process till the queue becomes empty:
    // - Remove the first vertex of the queue.
    while let Some((len, state)) = queue.pop_front() {
        if len >= time {
            answer = std::cmp::max(answer, state.geodes);
            continue;
        }

        // - Mark that vertex as visited.Mark that vertex as visited.
        if !seen.insert(state) {
            continue;
        }

        // - Insert all the unvisited neighbours of the vertex into the queue.
        if state.ore >= blueprint.geode_robot_ore_cost && state.obsidian >= blueprint.geode_robot_obsidian_cost {
            let mut state = state.collect_resources();
            state.ore -= blueprint.geode_robot_ore_cost;
            state.obsidian -= blueprint.geode_robot_obsidian_cost;
            state.geode_robots += 1;
            queue.push_back((len+1, state));
        }

        if state.obsidian_robots < blueprint.geode_robot_obsidian_cost && state.ore >= blueprint.obsidian_robot_ore_cost && state.clay >= blueprint.obsidian_robot_clay_cost {
            let mut state = state.collect_resources();
            state.ore -= blueprint.obsidian_robot_ore_cost;
            state.clay -= blueprint.obsidian_robot_clay_cost;
            state.obsidian_robots += 1;
            queue.push_back((len+1, state));
        }

        if state.ore_robots < max_ore_cost && state.ore >= blueprint.ore_robot_ore_cost {
            let mut state = state.collect_resources();
            state.ore -= blueprint.ore_robot_ore_cost;
            state.ore_robots += 1;
            queue.push_back((len+1, state));
        }

        if state.clay_robots < blueprint.obsidian_robot_clay_cost && state.ore >= blueprint.clay_robot_ore_cost {
            let mut state = state.collect_resources();
            state.ore -= blueprint.clay_robot_ore_cost;
            state.clay_robots += 1;
            queue.push_back((len+1, state));
        }

        queue.push_back((len+1, state.collect_resources()));
    }
    answer
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

fn part_one(input: &String) -> u32 {
    let blueprints = input
        .lines()
        .map(|line| {
            let [
                id, 
                ore_robot_ore_cost, 
                clay_robot_ore_cost, 
                obsidian_robot_ore_cost, 
                obsidian_robot_clay_cost, 
                geode_robot_ore_cost, 
                geode_robot_obsidian_cost
            ] = extract_numbers(line);
            
            Blueprint {
                id: id as u32,
                ore_robot_ore_cost: ore_robot_ore_cost as u32,
                clay_robot_ore_cost: clay_robot_ore_cost as u32,
                obsidian_robot_ore_cost: obsidian_robot_ore_cost as u32,
                obsidian_robot_clay_cost: obsidian_robot_clay_cost as u32,
                geode_robot_ore_cost: geode_robot_ore_cost as u32,
                geode_robot_obsidian_cost: geode_robot_obsidian_cost as u32,
            }
        })
        .collect::<Vec<Blueprint>>();

    return blueprints
        .par_iter()
        .map(|blueprint| bfs(blueprint, 24) * blueprint.id)
        .sum();
}

fn part_two(input: &String) -> u32 {    
    let blueprints = input
        .lines()
        .map(|line| {
            let [
                id, 
                ore_robot_ore_cost, 
                clay_robot_ore_cost, 
                obsidian_robot_ore_cost, 
                obsidian_robot_clay_cost, 
                geode_robot_ore_cost, 
                geode_robot_obsidian_cost
            ] = extract_numbers(line);
            
            Blueprint {
                id: id as u32,
                ore_robot_ore_cost: ore_robot_ore_cost as u32,
                clay_robot_ore_cost: clay_robot_ore_cost as u32,
                obsidian_robot_ore_cost: obsidian_robot_ore_cost as u32,
                obsidian_robot_clay_cost: obsidian_robot_clay_cost as u32,
                geode_robot_ore_cost: geode_robot_ore_cost as u32,
                geode_robot_obsidian_cost: geode_robot_obsidian_cost as u32,
            }
        })
        .collect::<Vec<Blueprint>>();

    return blueprints
        .par_iter()
        .take(3)
        .map(|blueprint| bfs(blueprint, 32))
        .product();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_the_test_answer_for_part_one() {
        let input = load_input("./fixtures/input_test.txt");
        assert_eq!(33, part_one(&input));
    }
    
    #[ignore]
    #[test]
    fn it_returns_the_test_answer_for_part_two() {
        let input = load_input("./fixtures/input_test.txt");
        assert_eq!(62, part_two(&input));
    }
    
    #[ignore]
    #[test]
    fn it_returns_the_answer_for_part_one() {
        let input = load_input("./fixtures/input.txt");
        assert_eq!(1389, part_one(&input));
    }
    
    #[ignore]
    #[test]
    fn it_returns_the_answer_for_part_two() {
        let input = load_input("./fixtures/input.txt");
        assert_eq!(3003, part_two(&input));
    }
}
