use std::time::Instant;

use crate::utils;

const DAY: u8 = 1;

pub fn solve() {
    let input = utils::read_lines(DAY, false);

    let start = Instant::now();
    let part1 = solve_part1(&input);
    let duration1 = start.elapsed();

    let start = Instant::now();
    let part2 = solve_part2(&input);
    let duration2 = start.elapsed();
    utils::print_grid(DAY, part1 as usize, part2 as usize, duration1, duration2);
}

fn solve_part1(input: &[String]) -> i64 {
    let mut position: i32 = 50;
    let mut zero_count: u32 = 0;
    for rotation in input {
        position += get_movement(rotation);
        position = position.rem_euclid(100);
        if position == 0 {
            zero_count += 1;
        }
    }
    zero_count as i64
}

fn solve_part2(input: &[String]) -> i64 {
    let mut position: i32 = 50;
    let mut zero_count: u32 = 0;
    for rotation in input {
        let movement = get_movement(rotation);
        position += movement;

        if position <= 0 || position >= 100 {
            let pos_abs = position.abs();

            let mut additional_zeros = match position {
                0 => 1,
                pos if pos < 0 => (pos_abs / 100) as u32 + 1,
                _ => (pos_abs / 100) as u32,
            };

            if position < 0 && position == -movement.abs() {
                additional_zeros -= 1;
            }

            zero_count += additional_zeros;
        }
        position = position.rem_euclid(100);
    }
    zero_count as i64
}

fn get_movement(input: &str) -> i32 {
    match parse_direction(input) {
        Some(('L', length)) => -(length as i32),
        Some(('R', length)) => length as i32,
        _ => 0,
    }
}

fn parse_direction(s: &str) -> Option<(char, u32)> {
    if s.is_empty() {
        return None;
    }
    let dir = s.as_bytes()[0] as char;
    let num: u32 = s[1..].parse().ok()?;
    Some((dir, num))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        let input = utils::read_lines(DAY, true);

        let result = solve_part1(&input);
        assert_eq!(result, 3);
    }

    #[test]
    fn solve_part_1() {
        let input_lines = utils::read_lines(DAY, false);

        let part1 = solve_part1(&input_lines);
        assert_eq!(part1, 1071);
    }

    #[test]
    fn example_part_2() {
        let input = utils::read_lines(DAY, true);
        let result = solve_part2(&input);
        assert_eq!(result, 6);
    }

    #[test]
    fn solve_part_2() {
        let input_lines = utils::read_lines(DAY, false);

        let part2 = solve_part2(&input_lines);
        assert_eq!(part2, 6700);
    }
}
