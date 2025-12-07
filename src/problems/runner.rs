use crate::problems::{aocday, day00, day01, day02, day03, day04, day05};
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
        // AoCDay {
        //     day: 6,
        //     part1: Box::new(day06::part1::execute),
        //     part2: Box::new(day06::part2::execute)
        // },
        // AoCDay {
        //     day: 7,
        //     part1: Box::new(day07::part1::execute),
        //     part2: Box::new(day07::part2::execute)
        // },
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
        // },
        // AoCDay {
        //     day: 13,
        //     part1: Box::new(day13::part1::execute),
        //     part2: Box::new(day13::part2::execute)
        // },
        // AoCDay {
        //     day: 14,
        //     part1: Box::new(day14::part1::execute),
        //     part2: Box::new(day14::part2::execute)
        // },
        // AoCDay {
        //     day: 15,
        //     part1: Box::new(day15::part1::execute),
        //     part2: Box::new(day15::part2::execute)
        // },
        // AoCDay {
        //     day: 16,
        //     part1: Box::new(day16::part1::execute),
        //     part2: Box::new(day16::part2::execute)
        // },
        // AoCDay {
        //     day: 17,
        //     part1: Box::new(day17::part1::execute),
        //     part2: Box::new(day17::part2::execute)
        // },
        // AoCDay {
        //     day: 18,
        //     part1: Box::new(day18::part1::execute),
        //     part2: Box::new(day18::part2::execute)
        // },
        // AoCDay {
        //     day: 19,
        //     part1: Box::new(day19::part1::execute),
        //     part2: Box::new(day19::part2::execute)
        // },
        // AoCDay {
        //     day: 20,
        //     part1: Box::new(day20::part1::execute),
        //     part2: Box::new(day20::part2::execute)
        // },
        // AoCDay {
        //     day: 21,
        //     part1: Box::new(day21::part1::execute),
        //     part2: Box::new(day21::part2::execute)
        // },
        // AoCDay {
        //     day: 22,
        //     part1: Box::new(day22::part1::execute),
        //     part2: Box::new(day22::part2::execute)
        // },
        // AoCDay {
        //     day: 23,
        //     part1: Box::new(day23::part1::execute),
        //     part2: Box::new(day23::part2::execute)
        // },
    ]
}