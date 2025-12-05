pub mod part1 {
    use std::fs;

    pub fn execute(input_path: &std::path::PathBuf) {
        let contents = fs::read_to_string(input_path).expect("Should have been able to read the file");
        let mut result = 0;
        for entry in contents.split(',').filter(|x| !x.is_empty()) {
            let mut sequence = entry.trim().split('-');
            let first = sequence.next().unwrap().parse::<i64>().unwrap();
            let second = sequence.next().unwrap().parse::<i64>().unwrap();

            for x in first..=second {
                let as_string = x.to_string();
                if as_string.len() % 2 == 0 {
                    // possible to be a mirror
                    // split it in half
                    let (part1, part2) = as_string.split_at(as_string.len() / 2);
                    if part1 == part2 {
                        result += x;
                    }
                }
            }
        }
        println!("Part 1: {result}");
    }
}

pub mod part2 {
    use std::fs;

    fn is_mirrored(word: &str) -> bool {
        let character_list: Vec<char> = word.chars().collect();
        for groups_count in 2..=character_list.len() {
            if character_list.len() % groups_count != 0 {
                continue; // not divisible
            }

            let group_size = character_list.len() / groups_count;
            let mut is_repeat = true;
            for word_idx in 1..groups_count {
                let start = group_size * word_idx;
                let end = start + group_size;
                is_repeat = is_repeat && character_list[0..group_size] == character_list[start..end];
            }

            if is_repeat {
                return true;
            }
        }

        false
    }

    pub fn execute(input_path: &std::path::PathBuf) {
        let contents = fs::read_to_string(input_path).expect("Should have been able to read the file");
        let mut result = 0;
        for entry in contents.split(',').filter(|x| !x.is_empty()) {
            let mut sequence = entry.trim().split('-');
            let first = sequence.next().unwrap().parse::<i64>().unwrap();
            let second = sequence.next().unwrap().parse::<i64>().unwrap();

            for x in first..=second {
                let as_string = x.to_string();
                if is_mirrored(&as_string) {
                    result += x;
                }
            }
        }
        println!("Part 2: {result}");
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn is_repeat() {
            let tests = vec!["2121212121", "824824824", "11", "22", "99", "111", "999", "1010", "1188511885", "222222", "446446"];
            tests.iter().for_each(|t| {
                println!("{t}");
                assert!(is_mirrored(t));
            });
        }
    }
}