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
];

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut day: u8 = 0;

    if args.len() == 2 {
        day = args[1].parse().expect("Tag muss eine Zahl sein");
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
    for &(_, solver) in AVAILABLE_DAYS {
        solver();
    }
}
