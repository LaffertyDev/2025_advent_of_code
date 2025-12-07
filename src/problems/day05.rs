mod shared {
    pub fn parse_input(contents: &str) -> (Vec<(i64, i64)>, Vec<i64>) {
        let mut lines = contents.lines();
        let mut ranges = vec![];
        while let Some(l) = lines.next() {
            if l.is_empty() {
                break; // now parse ingredients
            }

            let mut range = l.split('-');
            let start = range.next().unwrap().parse::<i64>().unwrap();
            let finish = range.next().unwrap().parse::<i64>().unwrap();
            ranges.push((start, finish));
        }

        let mut ingredients = vec![];
        while let Some(l) = lines.next() {
            if l.is_empty() {
                break;
            }

            ingredients.push(l.parse::<i64>().unwrap());
        }

        (ranges, ingredients)
    }

    pub fn is_in_range(range: &(i64, i64), test: &i64) -> bool {
        let (start, finish) = range;
        start <= test && test <= finish
    }

    pub fn reduce_ranges(mut ranges: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
        ranges.sort_by(|(start, _), (start2, _)| start.cmp(start2));
        // 1,2,3
        //   2,3
        //     3,4,5
        //           7,8,0

        // result ->
        // 1,2,3,4,5
        // 7,8,9

        let mut reduced_ranges = vec![];
        let mut x = 0;
        while x < ranges.len() {
            let (start, mut end) = ranges[x];

            let mut lookahead = x+1;
            while lookahead < ranges.len() {
                let (lookahead_start, lookahead_end) = ranges[lookahead];
                if lookahead_start <= end {
                    end = std::cmp::max(end, lookahead_end);
                } else {
                    break;
                }

                lookahead += 1;
            }

            x = lookahead;
            reduced_ranges.push((start, end));
        }

        reduced_ranges
    }

    pub fn count_range_elements(range: &(i64, i64)) -> i64 {
        let (start, end) = range;
        end - start + 1
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn simple() {
            let test_ranges = vec![(1,3), (2,3), (3,5), (7,9)];
            let reduced = reduce_ranges(test_ranges);
            assert_eq!(reduced.len(), 2);
        }
    }
}

pub mod part1 {
    use std::fs;
    use crate::problems::day05::shared::{is_in_range, parse_input};

    pub fn execute(input_path: &std::path::PathBuf) {
        let contents = fs::read_to_string(input_path).expect("Should have been able to read the file");
        let (ranges, ingredients) = parse_input(&contents);

        let count_spoiled = ingredients.iter().filter(|ingredient| ranges.iter().any(|r| is_in_range(r, ingredient))).count();
        println!("Part 1: {count_spoiled}");
    }
}

pub mod part2 {
    use std::fs;
    use crate::problems::day05::shared::{count_range_elements, parse_input, reduce_ranges};

    pub fn execute(input_path: &std::path::PathBuf) {
        let contents = fs::read_to_string(input_path).expect("Should have been able to read the file");
        let (ranges, _) = parse_input(&contents);
        let reduced_ranges = reduce_ranges(ranges);
        let count_fresh: i64 = reduced_ranges.iter().map(|r| count_range_elements(r)).sum();
        println!("Part 2: {count_fresh}");
    }
}