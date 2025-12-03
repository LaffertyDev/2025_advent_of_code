mod day01shared {
    pub fn compute_revolutions(lines: &Vec<&str>) -> (i32, i32) {
        let parsed_commands: Vec<Command> = lines.iter().filter(|l| !l.is_empty()).map(|l| {
            let mut chars = l.trim().chars();
            let d = chars.next().unwrap();
            let direction: Direction = if d == 'L' { Direction::Left } else { Direction::Right };
            let mut amount: i32 = 0;
            while let Some(digit) = chars.next() {
                amount *= 10;
                amount += digit.to_digit(10).unwrap() as i32;
            }

            Command {
                direction,
                amount
            }
        }).collect();

        let mut current = 50;
        let mut ticks = 0;
        let mut ticks_r2 = 0;

        for command in parsed_commands.iter() {
            match command.direction {
                Direction::Left => {
                    ticks_r2 += (command.amount / 100) + if command.amount % 100 >= current && current > 0 { 1 } else { 0 };
                    current = (100 + current - command.amount % 100) % 100;
                },
                Direction::Right => {
                    ticks_r2 += (command.amount + current) / 100;
                    current = (command.amount + current) % 100;
                }
            }

            if current == 0 {
                ticks += 1;
            }
        }

        (ticks, ticks_r2)
    }

    #[derive(Debug)]
    enum Direction {
        Left,
        Right
    }

    #[derive(Debug)]
    struct Command {
        direction: Direction,
        amount: i32
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_add() {
            let test_input: Vec<&str> = "L68
    L30
    R48
    L5
    R60
    L55
    L1
    L99
    R14
    L82".lines().collect();

            let (ticks1, ticks2) = compute_revolutions(&test_input);
            assert_eq!(ticks1, 3);
            assert_eq!(ticks2, 6);
        }
    }

}

pub mod part1 {
    use std::fs;

    use crate::problems::day01::day01shared;

    pub fn execute(input_path: &std::path::PathBuf) {
        let contents = fs::read_to_string(input_path).expect("Should have been able to read the file");
        let lines: Vec<&str> = contents.lines().collect();
        let (ticks, _) = day01shared::compute_revolutions(&lines);
        println!("Part 1: {ticks}");
    }
}

pub mod part2 {
    use std::fs;
    use crate::problems::day01::day01shared;

    pub fn execute(input_path: &std::path::PathBuf) {
        let contents = fs::read_to_string(input_path).expect("Should have been able to read the file");
        let lines: Vec<&str> = contents.lines().collect();
        let (_, ticks) = day01shared::compute_revolutions(&lines);
        println!("Part 2: {ticks}");
    }
}