pub mod part1 {
    use std::fs;

    fn is_grid_paper(grid: &Vec<Vec<bool>>, x: usize, y: usize, delta: (i64, i64)) -> bool {
        let (x_delta, y_delta) = delta;

        if x == 0 && x_delta < 0 {
            return false;
        }

        if y == 0 && y_delta < 0 {
            return false;
        }

        // upper bound for x and y

        if let Some(row) = grid.get((x as i64 + x_delta) as usize) {
            if let Some(col) = row.get((y as i64 + y_delta) as usize) {
                return *col;
            }
        }

        false
    }

    pub fn execute(input_path: &std::path::PathBuf) {
        let contents = fs::read_to_string(input_path).expect("Should have been able to read the file");
        let lines = contents.lines();

        let grid: Vec<Vec<bool>> = lines.filter(|l| !l.is_empty()).map(|l| {
            return l.chars().map(|c| {
                return c == '@';
            }).collect::<Vec<bool>>();
        }).collect();

        let mut accessible = 0;

        for x_pos in 0..grid.len() {
            let row_length = grid.get(x_pos).unwrap().len();
            for y_pos in 0..row_length {
                let directions = vec![
                    (-1, 0),
                    (-1, -1),
                    (-1, 1),
                    (1, 0),
                    (1, -1),
                    (1, 1),
                    (0, -1),
                    (0, 1)
                ];

                if grid[x_pos][y_pos] {
                    let count_rolls_cardinal = directions
                        .iter()
                        .filter(|d| is_grid_paper(&grid, x_pos, y_pos, **d))
                        .count();

                    if count_rolls_cardinal < 4 {
                        accessible += 1;
                    }
                }

            }
        }

        println!("Part 1: {accessible}");
    }
}

pub mod part2 {
    use std::fs;

    fn is_grid_paper(grid: &Vec<Vec<bool>>, x: usize, y: usize, delta: (i64, i64)) -> bool {
        let (x_delta, y_delta) = delta;

        if x == 0 && x_delta < 0 {
            return false;
        }

        if y == 0 && y_delta < 0 {
            return false;
        }

        // upper bound for x and y

        if let Some(row) = grid.get((x as i64 + x_delta) as usize) {
            if let Some(col) = row.get((y as i64 + y_delta) as usize) {
                return *col;
            }
        }

        false
    }

    pub fn execute(input_path: &std::path::PathBuf) {
        let contents = fs::read_to_string(input_path).expect("Should have been able to read the file");
        let lines = contents.lines();

        let mut grid: Vec<Vec<bool>> = lines.filter(|l| !l.is_empty()).map(|l| {
            return l.chars().map(|c| {
                return c == '@';
            }).collect::<Vec<bool>>();
        }).collect();

        let mut did_remove_paper = true;
        let mut accessible = 0;
        while did_remove_paper {
            did_remove_paper = false;
            for x_pos in 0..grid.len() {
                let row_length = grid[0].len();
                for y_pos in 0..row_length {
                    let directions = vec![
                        (-1, 0),
                        (-1, -1),
                        (-1, 1),
                        (1, 0),
                        (1, -1),
                        (1, 1),
                        (0, -1),
                        (0, 1)
                    ];

                    if grid[x_pos][y_pos] {
                        let count_rolls_cardinal = directions
                            .iter()
                            .filter(|d| is_grid_paper(&grid, x_pos, y_pos, **d))
                            .count();

                        if count_rolls_cardinal < 4 {
                            // remove this paper
                            grid[x_pos][y_pos] = false;
                            did_remove_paper = true;
                            accessible += 1;
                        }
                    }
                }
            }
        }

        println!("Part 2: {accessible}");
    }
}