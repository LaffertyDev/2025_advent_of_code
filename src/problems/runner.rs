use crate::problems::{aocday, day00, day01, day02, day03, day04, day05, day06, day07};
use crate::problems::aocday::AoCDay;

pub fn get_days() -> Vec<aocday::AoCDay> {
    vec![
        AoCDay {
            day: 0,
            part1: Box::new(day00::part1::execute),
            part2: Box::new(day00::part2::execute)
        },
        AoCDay {
            day: 1,
            part1: Box::new(day01::part1::execute),
            part2: Box::new(day01::part2::execute)
        },
        AoCDay {
            day: 2,
            part1: Box::new(day02::part1::execute),
            part2: Box::new(day02::part2::execute)
        },
        AoCDay {
            day: 3,
            part1: Box::new(day03::part1::execute),
            part2: Box::new(day03::part2::execute)
        },
        AoCDay {
            day: 4,
            part1: Box::new(day04::part1::execute),
            part2: Box::new(day04::part2::execute)
        },
        AoCDay {
            day: 5,
            part1: Box::new(day05::part1::execute),
            part2: Box::new(day05::part2::execute)
        },
        AoCDay {
            day: 6,
            part1: Box::new(day06::part1::execute),
            part2: Box::new(day06::part2::execute)
        },
        AoCDay {
            day: 7,
            part1: Box::new(day07::part1::execute),
            part2: Box::new(day07::part2::execute)
        },
        // AoCDay {
        //     day: 8,
        //     part1: Box::new(day08::part1::execute),
        //     part2: Box::new(day08::part2::execute)
        // },
        // AoCDay {
        //     day: 9,
        //     part1: Box::new(day09::part1::execute),
        //     part2: Box::new(day09::part2::execute)
        // },
        // AoCDay {
        //     day: 10,
        //     part1: Box::new(day10::part1::execute),
        //     part2: Box::new(day10::part2::execute)
        // },
        // AoCDay {
        //     day: 11,
        //     part1: Box::new(day11::part1::execute),
        //     part2: Box::new(day11::part2::execute)
        // },
        // AoCDay {
        //     day: 12,
        //     part1: Box::new(day12::part1::execute),
        //     part2: Box::new(day12::part2::execute)
        // }
    ]
}