#![allow(unused_imports)]

use aoc::*;

static DAY: &'static str = "15";

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct Sensor {
    x: i64,
    y: i64,
    radius: i64,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
pub struct Beacon {
    x: i64,
    y: i64,
}

fn main() {
    let input = load_input(format!("./2022/day_{DAY}/fixtures/input.txt"));

    let result1 = part_one(&input);
    println!("Answer day {DAY} part one: {result1}");

    let result2 = part_two(&input);
    println!("Answer day {DAY} part one: {result2}");
}

fn part_one(input: &String) -> usize {
    let mut sensors: HashSet<Loc> = HashSet::new();
    let mut beacons: HashSet<Loc> = HashSet::new();
    let mut no_beacons: HashSet<i64> = HashSet::new();
    let target_row = if DEBUG { 10 } else { 2000000 };

    // Load map
    for line in input.lines() {
        let [x1, y1, x2, y2] = extract_numbers(line);
        let sensor = Loc { x: x1, y: y1 };
        let beacon = Loc { x: x2, y: y2 };

        sensors.insert(sensor);
        if beacon.y == target_row {
            beacons.insert(beacon);
        }

        dbg!(&sensor, &beacon);

        // Get the distance from the sensor to the beacon
        // Basically a straight line over x
        let distance = (beacon - sensor).manhattan_distance();
        // Get distance from beacon to shortest location x
        // Basically a straight line over y
        let closest_y = distance - (sensor.y - target_row).abs();

        // Draw line
        for x in 0..=closest_y {
            // Store only the x locations, the y is the target_row
            no_beacons.insert(sensor.x + x as i64);
            no_beacons.insert(sensor.x - x as i64);
        }
    }

    return no_beacons.len() - beacons.len();
}

fn part_two(_input: &String) -> isize {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_the_answer_for_part_one() {
        let input = load_input("./fixtures/input_test.txt");
        assert_eq!(26, part_one(&input));
    }

    #[ignore]
    #[test]
    fn it_returns_the_answer_for_part_two() {
        let input = load_input("./fixtures/input_test.txt");
        assert_eq!(0, part_two(&input));
    }
}
