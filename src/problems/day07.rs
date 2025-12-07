pub mod part1 {
    use std::fs;

    pub fn execute(input_path: &std::path::PathBuf) {
        let contents = fs::read_to_string(input_path).expect("Should have been able to read the file");
        let grid = contents.lines().filter(|l| !l.is_empty())
            .map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

        let start = grid[0].iter().position(|l| *l == 'S').unwrap();

        let mut beams = vec![false; grid[0].len()];
        beams[start] = true;

        let mut splits = 0;
        for row in 0..grid.len() {
            for col in 0..beams.len() {
                if beams[col] && grid[row][col] == '^' {
                    splits += 1;
                    beams[col - 1] = true;
                    beams[col + 1] = true;
                    beams[col] = false;
                }
            }
        }

        println!("Part 1: {splits}");
    }
}

pub mod part2 {


    struct BeamRecord {
        row: usize,
        col: usize,
        parent_possibilities_count: i64
    }

    use std::collections::VecDeque;
    use std::fs;

    pub fn execute(input_path: &std::path::PathBuf) {
        let contents = fs::read_to_string(input_path).expect("Should have been able to read the file");
        let grid = contents.lines().filter(|l| !l.is_empty())
            .map(|l| l.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

        let start = grid[0].iter().position(|l| *l == 'S').unwrap();

        let mut beam_records: VecDeque <BeamRecord> = VecDeque::new();
        beam_records.push_front(BeamRecord {
            row: 0,
            col: start,
            parent_possibilities_count: 1
        });
        let mut total_possibilities = 0;

        while let Some(beam) = beam_records.pop_front() {
            // advance row by one, and check if there is a splitter
            let row_next = beam.row + 1;
            if row_next == grid.len() {
                // we have reached the end of the grid, no splitters can remain
                total_possibilities += beam.parent_possibilities_count;
                continue;
            } else if grid[row_next][beam.col] == '^' {
                if let Some(existing_beam) = beam_records.iter_mut().find(|f| f.row == row_next && f.col == beam.col - 1) {
                    existing_beam.parent_possibilities_count += beam.parent_possibilities_count;
                } else {
                    beam_records.push_back(BeamRecord {
                        row: row_next,
                        col: beam.col - 1,
                        parent_possibilities_count: beam.parent_possibilities_count
                    });
                }

                if let Some(existing_beam) = beam_records.iter_mut().find(|f| f.row == row_next && f.col == beam.col + 1) {
                    existing_beam.parent_possibilities_count *= beam.parent_possibilities_count;
                } else {
                    beam_records.push_back(BeamRecord {
                        row: row_next,
                        col: beam.col + 1,
                        parent_possibilities_count: beam.parent_possibilities_count
                    });
                }
            } else {
                // we are in empty space, simple re-insert case
                if let Some(existing_beam) = beam_records.iter_mut().find(|f| f.row == row_next && f.col == beam.col) {
                    existing_beam.parent_possibilities_count += beam.parent_possibilities_count;
                } else {
                    beam_records.push_back(BeamRecord {
                        row: row_next,
                        col: beam.col,
                        parent_possibilities_count: beam.parent_possibilities_count
                    });
                }
            }
        }

        println!("Part 2: {total_possibilities}");
    }
}