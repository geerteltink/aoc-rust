use aoc::*;

static DAY: &'static str = "25";

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

fn part_one(input: &String) -> String {
    let mut sum = input
        .lines()
        .map(|line| snafu_to_decimal(line))
        .sum::<i64>();

    let answer = decimal_to_snafu(sum);

    dbg!(&sum, &answer);

    return answer;
}

fn part_two(_input: &String) -> isize {
    return 0;
}

fn snafu_to_decimal(input: &str) -> i64 {
    return input.chars().fold(0, |acc, c| {
        acc * 5
            + match c {
                '=' => -2,
                '-' => -1,
                '0' => 0,
                '1' => 1,
                '2' => 2,
                _ => panic!(),
            }
    });
}

fn decimal_to_snafu(input: i64) -> String {
    let mut decimal = input;
    let mut digits = ['0'; 27];

    for c in &mut digits {
        let x = decimal + 2;
        let digit = x.rem_euclid(5) - 2;
        decimal = x.div_euclid(5);
        *c = match digit {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => panic!(),
        };
        if input == 0 {
            break;
        }
    }

    let mut result = String::new();
    for c in digits.into_iter().rev().skip_while(|&c| c == '0') {
        result.push(c);
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_the_decimal_for_a_snafu_number() {
        assert_eq!(1747, snafu_to_decimal("1=-0-2"));
        assert_eq!(906, snafu_to_decimal("12111"));
        assert_eq!(198, snafu_to_decimal("2=0="));
        assert_eq!(11, snafu_to_decimal("21"));
        assert_eq!(201, snafu_to_decimal("2=01"));
        assert_eq!(31, snafu_to_decimal("111"));
        assert_eq!(1257, snafu_to_decimal("20012"));
        assert_eq!(32, snafu_to_decimal("112"));
        assert_eq!(353, snafu_to_decimal("1=-1="));
        assert_eq!(107, snafu_to_decimal("1-12"));
        assert_eq!(7, snafu_to_decimal("12"));
        assert_eq!(3, snafu_to_decimal("1="));
        assert_eq!(37, snafu_to_decimal("122"));
    }

    #[test]
    fn it_returns_the_snafu_number_for_a_decimal() {
        assert_eq!(decimal_to_snafu(1), "1".to_string());
        assert_eq!(decimal_to_snafu(2), "2".to_string());
        assert_eq!(decimal_to_snafu(3), "1=".to_string());
        assert_eq!(decimal_to_snafu(4), "1-".to_string());
        assert_eq!(decimal_to_snafu(5), "10".to_string());
        assert_eq!(decimal_to_snafu(6), "11".to_string());
        assert_eq!(decimal_to_snafu(7), "12".to_string());
        assert_eq!(decimal_to_snafu(8), "2=".to_string());
        assert_eq!(decimal_to_snafu(9), "2-".to_string());
        assert_eq!(decimal_to_snafu(10), "20".to_string());
        assert_eq!(decimal_to_snafu(15), "1=0".to_string());
        assert_eq!(decimal_to_snafu(20), "1-0".to_string());
        assert_eq!(decimal_to_snafu(2022), "1=11-2".to_string());
        assert_eq!(decimal_to_snafu(12345), "1-0---0".to_string());
        assert_eq!(decimal_to_snafu(314159265), "1121-1110-1=0".to_string());
    }

    #[test]
    fn it_returns_the_test_answer_for_part_one() {
        let input = load_input("./fixtures/input_test.txt");
        assert_eq!("2=-1=0", part_one(&input));
    }

    #[test]
    fn it_returns_the_answer_for_part_one() {
        let input = load_input("./fixtures/input.txt");
        assert_eq!("2=020-===0-1===2=020", part_one(&input));
    }

    #[ignore]
    #[test]
    fn it_returns_the_test_answer_for_part_two() {
        let input = load_input("./fixtures/input_test.txt");
        assert_eq!(0, part_two(&input));
    }

    #[ignore]
    #[test]
    fn it_returns_the_answer_for_part_two() {
        let input = load_input("./fixtures/input.txt");
        assert_eq!(0, part_two(&input));
    }
}
