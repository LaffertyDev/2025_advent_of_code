mod shared {
}


pub mod part1 {
    use std::fs;

    pub fn execute(input_path: &std::path::PathBuf) {
        let contents = fs::read_to_string(input_path).expect("Should have been able to read the file");

        let problem_lines: Vec<Vec<&str>> = contents
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| l.split_whitespace().collect::<Vec<&str>>())
            .collect();
        let worksheet_problem_count = problem_lines[0].len();

        let mut result_summation = 0;
        for problem_index in 0..worksheet_problem_count {
            let operation_char = problem_lines[problem_lines.len() - 1][problem_index];

            let mut result: i64 = problem_lines[0][problem_index].parse::<i64>().unwrap();
            for row in 1..(problem_lines.len() - 1) {
                let number = problem_lines[row][problem_index].parse::<i64>().unwrap();
                match operation_char {
                    "+" => result += number,
                    "*" => result *= number,
                    _ => panic!("Invalid Math Operation")
                }
            }

            result_summation += result;
        }
        println!("Part 1: {result_summation}");
    }
}

pub mod part2 {
    use std::fs;

    #[derive(Copy, Clone)]
    enum ProblemSymbol {
        Add,
        Multiply
    }
    struct Column {
        num: i64,
        symbol: Option<ProblemSymbol>
    }

    struct ProblemGroup {
        numbers: Vec<i64>,
        operation: ProblemSymbol
    }

    impl ProblemGroup {
        fn parse(problem_columns: &Vec<&Column>) -> ProblemGroup {
            let mut discovered_operation: Option<ProblemSymbol> = None;
            let mut numbers = vec![];

            for p in problem_columns {
                if let Some(operation) =&p.symbol {
                    discovered_operation = Some(*operation);
                }

                numbers.push(p.num);
            }

            ProblemGroup {
                numbers,
                operation: discovered_operation.unwrap()
            }
        }

        fn solve(&self) -> i64 {
            let mut result: i64 = self.numbers[0];
            for row in 1..(self.numbers.len()) {
                match self.operation {
                    ProblemSymbol::Add => result += self.numbers[row],
                    ProblemSymbol::Multiply => result *= self.numbers[row]
                }
            }

            result
        }
    }

    fn parse_column(grid: &Vec<Vec<char>>, column: usize) -> Option<Column> {
        let mut parsed_operation: Option<ProblemSymbol> = None;
        let mut working_number: Option<i64> = None;
        for row in 0..grid.len() {
            if column >= grid[row].len() {
                // input bug.
                // the end of the column might not have whitespace.
                continue;
            }

            match grid[row][column] {
                ' ' => {
                    continue;
                },
                '*' => {
                    parsed_operation = Some(ProblemSymbol::Multiply);
                },
                '+' => {
                    parsed_operation = Some(ProblemSymbol::Add);
                },
                number => {
                    let num = number.to_digit(10).unwrap() as i64;
                    if let Some(current) = working_number {
                        working_number = Some(current * 10 + num);
                    } else {
                        working_number = Some(num);
                    }
                }
            }
        }

        if let Some(number) = working_number {
            Some(Column {
                num: number,
                symbol: parsed_operation
            })
        } else {
            None
        }
    }

    fn parse_to_groups(columns: Vec<Option<Column>>) -> Vec<ProblemGroup> {
        let mut working_columns: Vec<&Column> = vec![];
        let mut result = vec![];
        for c in 0..columns.len() {
            if let Some(c) = &columns[c] {
                working_columns.push(c);
            } else {
                result.push(ProblemGroup::parse(&working_columns));
                working_columns.clear();
            }
        }

        if !working_columns.is_empty() {
            result.push(ProblemGroup::parse(&working_columns));
        }


        result
    }

    fn solve_problem(contents: &str) -> i64 {
        let grid =
            contents.lines().filter(|l| !l.is_empty())
                .map(|l| l.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>();

        let num_columns = grid.iter().map(|l| l.len()).max().unwrap();

        let columns: Vec<Option<Column>> = (0..num_columns).rev().map(|c| parse_column(&grid, c)).collect();
        let problem_groups = parse_to_groups(columns);
        let result: i64 = problem_groups.iter().map(|p| p.solve()).sum();
        result
    }

    pub fn execute(input_path: &std::path::PathBuf) {
        let contents = fs::read_to_string(input_path).expect("Should have been able to read the file");
        let result = solve_problem(&contents);
        println!("Part 2: {result}");
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn is_repeat() {
            let test = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";

            assert_eq!(3263827, solve_problem(test));
        }
    }
}