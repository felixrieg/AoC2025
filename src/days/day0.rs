use crate::utils;

const DAY: u8 = 0;

pub fn solve() {
    println!("Löse Tag {}...", DAY);
    let input_line: String = utils::read_input_and_split(DAY, ",");

    let part1 = solve_part1(&inputs);
    let part2 = solve_part2(&inputs);

    println!("  Teil 1: {}", part1);
    println!("  Teil 2: {}", part2);
}

fn solve_part1(input: &Vec<String>) -> usize {
    input.len()
}

fn solve_part2(input: &Vec<String>) -> usize {
    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_all() {
        solve();
    }

    #[test]
    fn test_part_1() {
        let input = vec!["line".to_string()];
        let result = solve_part1(&input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_solve_part_1() {
        let input_lines = utils::read_lines(DAY);

        let part1 = solve_part1(&input_lines);
        assert_eq!(part1, 17109);
    }

    #[test]
    fn test_part_2() {
        let input = vec!["line".to_string()];
        let result = solve_part2(&input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_solve_part_2() {
        let input_lines = utils::read_lines(DAY);

        let part2 = solve_part2(&input_lines);
        assert_eq!(part2, 169347417057382);
    }
}
