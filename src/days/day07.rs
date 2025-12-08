use std::collections::HashSet;
use std::time::Instant;

use crate::utils;

const DAY: u8 = 7;

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
    let mut used_splitters: HashSet<(usize, usize)> = HashSet::new();

    let matrix = utils::convert_to_matrix(input);
    let width = matrix[0].len();
    let start_index = matrix[0]
        .iter()
        .position(|&n| n == 'S')
        .expect("No start found");

    let mut current_beams: HashSet<usize> = HashSet::new();
    current_beams.insert(start_index);

    for (row, matrix_row) in matrix.iter().enumerate().skip(1) {
        let mut new_beams: HashSet<usize> = HashSet::new();
        for &beam in &current_beams {
            let cell = matrix_row[beam];
            match cell {
                '.' => {
                    new_beams.insert(beam);
                }
                '^' => {
                    used_splitters.insert((row, beam));
                    if beam > 0 {
                        new_beams.insert(beam - 1); // left
                    }
                    if beam + 1 < width {
                        new_beams.insert(beam + 1); // right
                    }
                }
                other => panic!("Unknown cell {} at row {} col {}", other, row, beam),
            }
        }
        current_beams = new_beams;
    }

    used_splitters.len()
}

fn solve_part2(input: &[String]) -> usize {
    let matrix = utils::convert_to_matrix(input);
    let width = matrix[0].len();
    let start_index = matrix[0]
        .iter()
        .position(|&n| n == 'S')
        .expect("No start found");

    let rows = matrix.len();
    let mut cache: Vec<Option<usize>> = vec![None; rows * width];

    traverse(1, start_index, &matrix, width, &mut cache) + 1
}

fn traverse(
    row: usize,
    beam: usize,
    matrix: &[Vec<char>],
    width: usize,
    cache: &mut [Option<usize>],
) -> usize {
    let rows = matrix.len();
    if row >= rows {
        return 0;
    }

    let idx = row * width + beam;
    if let Some(cached) = cache[idx] {
        return cached;
    }

    let res = match matrix[row][beam] {
        '.' => traverse(row + 1, beam, matrix, width, cache),
        '^' => {
            let mut total_splitters = 1usize;
            if beam > 0 {
                total_splitters += traverse(row + 1, beam - 1, matrix, width, cache);
            }
            if beam + 1 < width {
                total_splitters += traverse(row + 1, beam + 1, matrix, width, cache);
            }
            total_splitters
        }
        other => panic!("Unknown cell {} at row {} col {}", other, row, beam),
    };

    cache[idx] = Some(res);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn example_part_1() {
        let input = utils::read_lines(DAY, true);

        let result = solve_part1(&input);
        assert_eq!(result, 21);
    }

    #[test]
    #[ignore]
    fn solve_part_1() {
        let input_lines = utils::read_lines(DAY, false);

        let part1 = solve_part1(&input_lines);
        assert_eq!(part1, 1590);
    }

    #[test]
    #[ignore]
    fn example_part_2() {
        let input = utils::read_lines(DAY, true);
        let result = solve_part2(&input);
        assert_eq!(result, 40);
    }

    #[test]
    #[ignore]
    fn solve_part_2() {
        let input_lines = utils::read_lines(DAY, false);
        let part2 = solve_part2(&input_lines);
        assert_eq!(part2, 20571740188555);
    }
}
