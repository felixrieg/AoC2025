use std::time::Instant;

use crate::utils;

const DAY: u8 = 1;

pub fn solve() {
    let inputs = utils::read_lines(DAY, false);

    let start = Instant::now();
    let part1 = solve_part1(&inputs);
    let duration1 = start.elapsed();

    let start = Instant::now();
    let part2 = solve_part2(&inputs);
    let duration2 = start.elapsed();

    utils::print_grid(DAY, part1, part2, duration1, duration2);
}

fn solve_part1(input: &[String]) -> usize {
    0
}

fn solve_part2(input: &[String]) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        let input = utils::read_lines(DAY, true);

        let result = solve_part1(&input);
        assert_eq!(result, 0);
    }

    #[test]
    fn solve_part_1() {
        let input_lines = utils::read_lines(DAY, false);

        let part1 = solve_part1(&input_lines);
        assert_eq!(part1, 0);
    }

    #[test]
    fn example_part_2() {
        let input = utils::read_lines(DAY, true);
        let result = solve_part2(&input);
        assert_eq!(result, 0);
    }

    #[test]
    fn solve_part_2() {
        let input_lines = utils::read_lines(DAY, false);

        let part2 = solve_part2(&input_lines);
        assert_eq!(part2, 0);
    }
}
