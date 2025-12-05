use crate::utils;

const DAY: u8 = 1;

pub fn solve() {
    println!("Löse Tag {}...", DAY);
    let input = utils::read_lines(DAY);

    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);

    println!("  Teil 1: {}", part1);
    println!("  Teil 2: {}", part2);
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
    fn test_part_1() {
        let input = vec![
            "L68".to_string(),
            "L30".to_string(),
            "R48".to_string(),
            "L5".to_string(),
            "R60".to_string(),
            "L55".to_string(),
            "L1".to_string(),
            "L99".to_string(),
            "R14".to_string(),
            "L82".to_string(),
        ];
        let result = solve_part1(&input);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_solve_part_1() {
        let input = utils::read_lines(DAY);
        let part1 = solve_part1(&input);
        assert_eq!(part1, 1071);
    }

    #[test]
    fn test_part_2() {
        let input = vec![
            "L168".to_string(),
            "L30".to_string(),
            "R48".to_string(),
            "L5".to_string(),
            "R60".to_string(),
            "L55".to_string(),
            "L1".to_string(),
            "L99".to_string(),
            "R14".to_string(),
            "L82".to_string(),
        ];
        let result = solve_part2(&input);
        assert_eq!(result, 7);
    }

    #[test]
    fn test_solve_part_2() {
        let input = utils::read_lines(DAY);
        let part2 = solve_part2(&input);
        assert_eq!(part2, 6700);
    }
}
