use crate::{types::ranges::Range, utils};

const DAY: u8 = 5;

pub fn solve() {
    println!("Löse Tag {}...", DAY);

    let input = utils::read_lines(DAY);

    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);

    println!("  Teil 1: {}", part1);
    println!("  Teil 2: {}", part2);
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
    let split_inputs = utils::split_on_empty_lines(&input);
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
    fn test_part_1() {
        let input = vec![
            "3-5".to_string(),
            "10-14".to_string(),
            "16-20".to_string(),
            "12-18".to_string(),
            "".to_string(),
            "1".to_string(),
            "5".to_string(),
            "8".to_string(),
            "11".to_string(),
            "17".to_string(),
            "32".to_string(),
        ];
        let result = solve_part1(&input);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_solve_part_1() {
        let input = utils::read_lines(DAY);

        let part1 = solve_part1(&input);
        assert_eq!(part1, 615);
    }

    #[test]
    fn test_part_2() {
        let input = vec![
            "3-5".to_string(),
            "10-14".to_string(),
            "16-20".to_string(),
            "12-18".to_string(),
            "".to_string(),
            "1".to_string(),
            "5".to_string(),
            "8".to_string(),
            "11".to_string(),
            "17".to_string(),
            "32".to_string(),
        ];
        let result = solve_part2(&input);
        assert_eq!(result, 14);
    }

    #[test]
    fn test_solve_part_2() {
        let input = utils::read_lines(DAY);

        let part2 = solve_part2(&input);
        assert_eq!(part2, 353716783056994);
    }
}
