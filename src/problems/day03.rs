pub mod part1 {
    use std::fs;

    fn find_largest_rec(range: &[u32]) -> u32 {
        if range.len() == 2 {
            return range[0] * 10 + range[1];
        }

        let mut largest = range[0] * 10 + range[1];
        for x in &range[2..] {
            let joltage = range[0] * 10 + x;
            if joltage > largest {
                largest =joltage
            }
        }

        let sub_largest = find_largest_rec(&range[1.. ]);
        std::cmp::max(largest, sub_largest)
    }

    pub fn execute(input_path: &std::path::PathBuf) {
        let contents = fs::read_to_string(input_path).expect("Should have been able to read the file");

        let lines = contents.lines().filter(|l| !l.is_empty());

        let mut joltage = 0;
        for l in lines {
            let bank: Vec<u32> = l.chars().map(|c| c.to_digit(10).unwrap()).collect();
            joltage += find_largest_rec(&bank);
        }

        println!("Part 1: {joltage}");
    }
}

pub mod part2 {
    use std::collections::HashMap;
    use std::fs;

    fn find_largest_jolt(range: &Vec<u64>, num_take: usize) -> u64 {
        let mut oracle = HashMap::new();
        find_largest_rec_size(range, 0, num_take, &mut oracle)
    }

    fn find_largest_rec_size(range: &Vec<u64>, start_idx: usize, num_take: usize, oracle: &mut HashMap::<(usize, usize), u64>) -> u64 {
        if let Some(res) = oracle.get(&(start_idx, num_take)) {
            return *res;
        }

        if num_take == 1 {
            return range.iter().skip(start_idx).max().unwrap().clone();
        }

        let mut max = 0;
        for left_idx in start_idx..=(range.len()-num_take) {
            let left_val = range[left_idx] * 10u64.pow(num_take as u32 - 1);
            let right_bound = range.len() - (num_take - 1);

            for sub in (left_idx+1)..=right_bound {
                let sub_val = find_largest_rec_size(range, sub, num_take - 1, oracle);
                *oracle.entry((sub, num_take - 1)).or_insert(sub_val) = sub_val;
                let total = left_val + sub_val;
                if total >= max {
                    max = total;
                }
            }
        }

        if max == 0 {
            panic!();
        }

        max
    }

    pub fn execute(input_path: &std::path::PathBuf) {
        let contents = fs::read_to_string(input_path).expect("Should have been able to read the file");

        let lines = contents.lines().filter(|l| !l.is_empty());

        let mut joltage = 0;
        for l in lines {
            let bank: Vec<u64> = l.chars().map(|c| c.to_digit(10).unwrap() as u64).collect();
            joltage += find_largest_jolt(&bank, 12);
        }

        println!("Part 2: {joltage}");
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn simple() {
            let val = "1234".chars().map(|c| c.to_digit(10).unwrap() as u64).collect();
            assert_eq!(find_largest_jolt(&val, 1), 4);
            assert_eq!(find_largest_jolt(&val, 2), 34);
            assert_eq!(find_largest_jolt(&val, 3), 234);
            assert_eq!(find_largest_jolt(&val, 4), 1234);
        }
        #[test]
        fn doubles() {
            let val = "12341234".chars().map(|c| c.to_digit(10).unwrap() as u64).collect();
            assert_eq!(find_largest_jolt(&val, 1), 4);
            assert_eq!(find_largest_jolt(&val, 2), 44);
            assert_eq!(find_largest_jolt(&val, 3), 434);
            assert_eq!(find_largest_jolt(&val, 4), 4234);
        }

        #[test]
        fn test() {
            let val = "234234234234278".chars().map(|c| c.to_digit(10).unwrap() as u64).collect();
            assert_eq!(find_largest_jolt(&val, 12), 434234234278);
        }
    }
}