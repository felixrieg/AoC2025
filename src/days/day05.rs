use std::time::Instant;

use crate::{types::ranges::Range, utils};

const DAY: u8 = 5;

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

fn solve_part1(input: &[String]) -> usize {
    let (ranges, numbers) = map_to_ranges_and_numbers(input);

    numbers
        .iter()
        .filter(|n| in_any_range(**n, &ranges))
        .count()
}

fn solve_part2(input: &[String]) -> usize {
    let (ranges, _) = map_to_ranges_and_numbers(input);

    let merged_ranges = merge_ranges(&ranges);
    merged_ranges.iter().map(|r| r.size()).sum()
}

fn merge_ranges(ranges: &[Range]) -> Vec<Range> {
    let mut sorted_ranges: Vec<Range> = ranges.to_vec();
    sorted_ranges.sort_by_key(|r| r.start);

    let mut merged_ranges: Vec<Range> = Vec::new();

    for range in sorted_ranges {
        if let Some(last) = merged_ranges.last_mut() {
            if range.overlaps_with(last) {
                last.merge_into(&range);
            } else {
                merged_ranges.push(range);
            }
        } else {
            merged_ranges.push(range);
        }
    }

    merged_ranges
}

fn map_to_ranges_and_numbers(input: &[String]) -> (Vec<Range>, Vec<usize>) {
    let split_inputs = utils::split_on_empty_lines(input);
    assert_eq!(split_inputs.len(), 2);

    let ranges = convert_to_ranges(&split_inputs[0]);
    let numbers: Vec<usize> = split_inputs[1]
        .iter()
        .filter_map(|s| s.parse().ok())
        .collect();

    (ranges, numbers)
}

fn in_any_range(n: usize, ranges: &[Range]) -> bool {
    ranges.iter().any(|r| r.contains(n))
}

fn convert_to_ranges(input: &[String]) -> Vec<Range> {
    input
        .iter()
        .filter_map(|line| Range::from_string(line))
        .collect()
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
        assert_eq!(part1, 615);
    }

    #[test]
    fn example_part_2() {
        let input = utils::read_lines(DAY, true);
        let result = solve_part2(&input);
        assert_eq!(result, 14);
    }

    #[test]
    fn solve_part_2() {
        let input_lines = utils::read_lines(DAY, false);

        let part2 = solve_part2(&input_lines);
        assert_eq!(part2, 353716783056994);
    }
}
