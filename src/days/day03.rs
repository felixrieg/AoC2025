use std::time::Instant;

use crate::utils;

const DAY: u8 = 3;

pub fn solve() {
    let input_lines = utils::read_lines(DAY, false);

    let start = Instant::now();
    let part1 = solve_part1(&input_lines);
    let duration1 = start.elapsed();

    let start = Instant::now();
    let part2 = solve_part2(&input_lines);
    let duration2 = start.elapsed();
    utils::print_grid(DAY, part1 as usize, part2 as usize, duration1, duration2);
}

fn solve_part1(input: &[String]) -> usize {
    input
        .iter()
        .map(|line| parse_line(line))
        .map(|numbers| get_highest_number_of_size(&numbers, 2))
        .sum()
}

fn solve_part2(input: &[String]) -> usize {
    input
        .iter()
        .map(|line| parse_line(line))
        .map(|numbers| get_highest_number_of_size(&numbers, 12))
        .sum()
}

fn parse_line(line: &str) -> Vec<u8> {
    line.trim()
        .chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as u8))
        .collect()
}

fn get_highest_number_of_size(numbers: &[u8], size: usize) -> usize {
    if numbers.len() < size {
        return 0;
    }

    let mut digits: Vec<usize> = vec![0; size];
    let mut current_limit = 0;
    for hi in 0..digits.len() {
        let start = current_limit;
        for index in start..(numbers.len() - (size - hi - 1)) {
            if numbers[index] > numbers[digits[hi]] || (hi > 0 && digits[hi - 1] >= digits[hi]) {
                digits[hi] = index;
                current_limit = index + 1;
            }
        }
    }

    digits
        .iter()
        .enumerate()
        .map(|(hi, &digit)| (numbers[digit] as usize) * 10_usize.pow((size - hi - 1) as u32))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        let input = utils::read_lines(DAY, true);

        let result = solve_part1(&input);
        assert_eq!(result, 357);
    }

    #[test]
    fn solve_part_1() {
        let input_lines = utils::read_lines(DAY, false);

        let part1 = solve_part1(&input_lines);
        assert_eq!(part1, 17109);
    }

    #[test]
    fn example_part_2() {
        let input = utils::read_lines(DAY, true);
        let result = solve_part2(&input);
        assert_eq!(result, 3121910778619);
    }

    #[test]
    fn solve_part_2() {
        let input_lines = utils::read_lines(DAY, false);

        let part2 = solve_part2(&input_lines);
        assert_eq!(part2, 169347417057382);
    }
}
