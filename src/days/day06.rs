use std::time::Instant;

use crate::utils;

const DAY: u8 = 6;

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
    assert!(input.len() > 1);

    let operators_line = input.last().unwrap();
    let operators: Vec<&str> = operators_line.split_whitespace().collect();

    let data_rows = &input[..input.len() - 1];
    let rows = parse_rows(data_rows);

    let columns = transpose_rows(&rows);

    columns
        .iter()
        .enumerate()
        .map(|(i, col)| calculate_row_value(col, operators.get(i).copied().unwrap_or("")))
        .sum()
}

fn solve_part2(input: &[String]) -> usize {
    assert!(
        input.len() > 1,
        "expected at least one data line and an operator line"
    );
    let operators_line = input.last().unwrap();
    let operators: Vec<&str> = operators_line.split_whitespace().collect();

    let data_rows = &input[..input.len() - 1];
    let columns = slice_input_by_columns(data_rows);

    columns
        .iter()
        .enumerate()
        .map(|(i, col)| calculate_row_value(col, operators.get(i).copied().unwrap_or("")))
        .sum()
}

fn parse_rows(input_lines: &[String]) -> Vec<Vec<usize>> {
    input_lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<usize>().unwrap_or(0))
                .collect()
        })
        .collect()
}

fn transpose_rows(rows: &[Vec<usize>]) -> Vec<Vec<usize>> {
    if rows.is_empty() {
        return vec![];
    }
    let cols = rows[0].len();
    let mut out = vec![Vec::with_capacity(rows.len()); cols];
    for r in rows {
        for (c, &v) in r.iter().enumerate() {
            out[c].push(v);
        }
    }
    out
}

fn slice_input_by_columns(input: &[String]) -> Vec<Vec<usize>> {
    if input.is_empty() {
        return vec![];
    }

    let max_len = input.iter().map(|s| s.len()).max().unwrap_or(0);

    let mut result: Vec<Vec<usize>> = Vec::new();
    let mut current_column_numbers: Vec<usize> = Vec::new();

    for col_idx in 0..max_len {
        let mut vertical = String::with_capacity(input.len());
        for row in input {
            if let Some(ch) = row.chars().nth(col_idx) {
                vertical.push(ch);
            }
        }

        if vertical.trim().is_empty() {
            if !current_column_numbers.is_empty() {
                result.push(std::mem::take(&mut current_column_numbers));
            } else {
                break;
            }
        } else {
            let value = vertical.trim().parse::<usize>().unwrap_or(0);
            current_column_numbers.push(value);
        }
    }

    if !current_column_numbers.is_empty() {
        result.push(current_column_numbers);
    }

    result
}

fn calculate_row_value(row: &[usize], operator: &str) -> usize {
    match operator {
        "+" => row.iter().sum::<usize>(),
        "*" => row.iter().product::<usize>(),
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_part_1() {
        let input = utils::read_lines(DAY, true);

        let result = solve_part1(&input);
        assert_eq!(result, 4277556);
    }

    #[test]
    #[ignore]
    fn test_solve_part_1() {
        let input_lines = utils::read_lines(DAY, false);

        let part1 = solve_part1(&input_lines);
        assert_eq!(part1, 5782351442566);
    }

    #[test]
    #[ignore]
    fn test_part_2() {
        let input = utils::read_lines(DAY, true);
        let result = solve_part2(&input);
        assert_eq!(result, 3263827);
    }

    #[test]
    #[ignore]
    fn test_solve_part_2() {
        let input_lines = utils::read_lines(DAY, false);

        let part2 = solve_part2(&input_lines);
        assert_eq!(part2, 10194584711842);
    }
}
