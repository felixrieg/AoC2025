use crate::{types::ranges::Range, utils};

const DAY: u8 = 2;

pub fn solve() {
    println!("Löse Tag {}...", DAY);
    let inputs = utils::read_input_and_split(DAY, ",");

    let part1 = solve_part1(&inputs);
    let part2 = solve_part2(&inputs);

    println!("  Teil 1: {}", part1);
    println!("  Teil 2: {}", part2);
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
    len % 2 == 0 && has_number_repeates_of_size_fast(&s, len / 2)
}

fn is_number_repeated_any_size(n: &usize) -> bool {
    let s = n.to_string();
    let max_repeat_size = s.len() / 2;
    (1..=max_repeat_size).any(|size| has_number_repeates_of_size_fast(&s, size))
}

fn has_number_repeates_of_size_fast(s: &str, size: usize) -> bool {
    let bytes = s.as_bytes();

    if bytes.len() % size != 0 || bytes[0] == b'0' || bytes[size] == b'0' {
        return false;
    }

    bytes.len() > size && (0..bytes.len() - size).all(|i| bytes[i] == bytes[i + size])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = vec![
            "11-22".to_string(),
            "95-115".to_string(),
            "998-1012".to_string(),
            "1188511880-1188511890".to_string(),
            "222220-222224".to_string(),
            "1698522-1698528".to_string(),
            "446443-446449".to_string(),
            "38593856-38593862".to_string(),
            "565653-565659".to_string(),
            "824824821-824824827".to_string(),
            "2121212118-2121212124".to_string(),
        ];
        let result = solve_part1(&input);
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn test_solve_part_1() {
        let inputs = utils::read_input_and_split(DAY, ",");

        let part1 = solve_part1(&inputs);
        assert_eq!(part1, 18595663903);
    }

    #[test]
    fn test_part_2() {
        let input = vec![
            "11-22".to_string(),
            "95-115".to_string(),
            "998-1012".to_string(),
            "1188511880-1188511890".to_string(),
            "222220-222224".to_string(),
            "1698522-1698528".to_string(),
            "446443-446449".to_string(),
            "38593856-38593862".to_string(),
            "565653-565659".to_string(),
            "824824821-824824827".to_string(),
            "2121212118-2121212124".to_string(),
        ];
        let result = solve_part2(&input);
        assert_eq!(result, 4174379265);
    }

    #[test]
    fn test_solve_part_2() {
        let inputs = utils::read_input_and_split(DAY, ",");

        let part2 = solve_part2(&inputs);
        assert_eq!(part2, 19058204438);
    }
}
