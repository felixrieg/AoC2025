use std::time::Instant;

use crate::{types::ranges::Range, utils};

const DAY: u8 = 2;

pub fn solve() {
    let inputs = utils::read_input_and_split(DAY, false, ",");

    let start = Instant::now();
    let part1 = solve_part1(&inputs);
    let duration1 = start.elapsed();

    let start = Instant::now();
    let part2 = solve_part2(&inputs);
    let duration2 = start.elapsed();
    utils::print_grid(DAY, part1 as usize, part2 as usize, duration1, duration2);
}

fn solve_part1(input: &[String]) -> usize {
    solve_with(input, is_number_repeated)
}

fn solve_part2(input: &[String]) -> usize {
    solve_with(input, is_number_repeated_any_size)
}

fn solve_with(input: &[String], check_fn: fn(&usize) -> bool) -> usize {
    input
        .iter()
        .filter_map(|line| Range::from_string(line))
        .flat_map(|r| r.into_iter().filter(check_fn))
        .sum()
}

fn is_number_repeated(n: &usize) -> bool {
    let s = n.to_string();
    let len = s.len();
    len.is_multiple_of(2) && has_number_repeats_of_size_fast(&s, len / 2)
}

fn is_number_repeated_any_size(n: &usize) -> bool {
    let s = n.to_string();
    let max_repeat_size = s.len() / 2;
    (1..=max_repeat_size).any(|size| has_number_repeats_of_size_fast(&s, size))
}

fn has_number_repeats_of_size_fast(s: &str, size: usize) -> bool {
    let bytes = s.as_bytes();

    if !bytes.len().is_multiple_of(size) || bytes[0] == b'0' || bytes[size] == b'0' {
        return false;
    }

    bytes.len() > size && (0..bytes.len() - size).all(|i| bytes[i] == bytes[i + size])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        let input = utils::read_input_and_split(DAY, true, ",");

        let result = solve_part1(&input);
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn solve_part_1() {
        let input_lines = utils::read_input_and_split(DAY, false, ",");

        let part1 = solve_part1(&input_lines);
        assert_eq!(part1, 18595663903);
    }

    #[test]
    fn example_part_2() {
        let input = utils::read_input_and_split(DAY, true, ",");
        let result = solve_part2(&input);
        assert_eq!(result, 4174379265);
    }

    #[test]
    fn solve_part_2() {
        let input_lines = utils::read_input_and_split(DAY, false, ",");

        let part2 = solve_part2(&input_lines);
        assert_eq!(part2, 19058204438);
    }
}
