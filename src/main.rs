mod days;
mod types;
mod utils;

use std::{env, time::Instant};

const AVAILABLE_DAYS: &[(u8, fn())] = &[
    (1, days::day01::solve),
    (2, days::day02::solve),
    (3, days::day03::solve),
    (4, days::day04::solve),
    (5, days::day05::solve),
    (6, days::day06::solve),
    (7, days::day07::solve),
    (8, days::day08::solve),
    (9, days::day09::solve),
    (10, days::day10::solve),
    (11, days::day11::solve),
    (12, days::day12::solve),
];

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut day: u8 = 0;

    if args.len() == 2 {
        day = args[1].parse().expect("Day must be a number");
    }

    utils::print_grid_header();

    let start = Instant::now();
    match day {
        0 => solve_all(),
        i => solve(i),
    }
    let total_duration = start.elapsed();
    utils::print_time(total_duration);
}

fn solve(day: u8) {
    for &(day_num, solver) in AVAILABLE_DAYS {
        if day_num == day {
            solver();
            return;
        }
    }
}

fn solve_all() {
    AVAILABLE_DAYS.iter().for_each(|&(_, solver)| {
        solver();
    });
}
