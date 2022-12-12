static DAY: &'static str = "10";

type Int = i16;

#[derive(Debug, Clone, Copy)]
enum Instr {
    AddX(Int),
    Noop,
}

#[derive(Debug)]
struct CPU {
    x: Int,
    cycles: usize,
}

impl CPU {
    pub fn execute(&mut self, instr: &Instr) {
        match &instr {
            Instr::Noop => (),
            Instr::AddX(n) => {
                self.x += n;
            }
        }
    }
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            x: 1,
            cycles: 0,
        }
    }
}

fn main() {
    let input = std::fs::read_to_string(format!("./2022/day_{DAY}/fixtures/input.txt")).unwrap();

    let result1 = part_one(&input);
    println!("Answer day {DAY} part one: {result1}");

    let result2 = part_two(&input);
    println!("Answer day {DAY} part one: {result2}");
}

fn part_one(input: &String) -> i32 {
    let mut cpu = CPU::default();
    let parser = input.lines()
        .map(|line| match &line[0..4] {
            "addx" => (Instr::AddX(line[5..].parse::<Int>().unwrap()), 2),
            "noop" => (Instr::Noop, 1),
            _ => unreachable!(),
        })
        .flat_map(|(instr, cycle)| (0..cycle).rev().map(move |cycle| (instr, cycle)));

    let mut sigs = 0;

    for (instr, cycle) in parser {
        cpu.cycles += 1;
        if (cpu.cycles + 20) % 40 == 0 {
            sigs += cpu.cycles as i32 * cpu.x as i32;
        }
        if cycle == 0 {
            cpu.execute(&instr);
        }
    }
    
    return sigs;
}

fn part_two(input: &String) -> i32 {
    let mut cpu = CPU::default();
    let parser = input.lines()
        .map(|line| match &line[0..4] {
            "addx" => (Instr::AddX(line[5..].parse::<Int>().unwrap()), 2),
            "noop" => (Instr::Noop, 1),
            _ => unreachable!(),
        })
        .flat_map(|(instr, cycle)| (0..cycle).rev().map(move |cycle| (instr, cycle)));

    let mut sigs = 0;
    let mut crt = String::from("");

    for (instr, cycle) in parser {
        cpu.cycles += 1;
        
        if (cpu.cycles + 20) % 40 == 0 {
            sigs += cpu.cycles as i32 * cpu.x as i32;
        }
        
        if cycle == 0 {
            cpu.execute(&instr);
        }

        if (cpu.cycles - 1) % 40 == 0 {
            crt.push_str("\n");
        }
        
        if (cpu.cycles as i32 % 40 - cpu.x as i32).abs() < 2 {
            crt.push('x');
        } else {
            crt.push(' ');
        }
        
        // TODO: fix output -> there is a mistake somewhere the first column is shown on the right.
        //println!("{:?}, {:?} -> {:?}", cpu.cycles, cycle, &crt.pop());
    }
    
    println!("{}", crt);
    
    return sigs;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_the_answer_for_part_one() {
        let input = std::fs::read_to_string("./fixtures/input_test.txt").unwrap();
        assert_eq!(13140, part_one(&input));
    }

    #[test]
    fn it_returns_the_answer_for_part_two() {
        let input = std::fs::read_to_string("./fixtures/input_test.txt").unwrap();
        assert_eq!(13140, part_two(&input));
    }
}
