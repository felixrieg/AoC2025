use std::time::Instant;

use crate::utils;

const DAY: u8 = 4;

pub fn solve() {
    let inputs = utils::read_lines(DAY, false);

    let start = Instant::now();
    let part1 = solve_part1(&inputs);
    let duration1 = start.elapsed();

    let start = Instant::now();
    let part2 = solve_part2(&inputs);
    let duration2 = start.elapsed();
    utils::print_grid(DAY, part1 as usize, part2 as usize, duration1, duration2);
}

fn solve_part1(input: &[String]) -> usize {
    let grid = map_to_number_grid(input);
    get_indices_for_removal(&grid).len()
}

fn solve_part2(input: &[String]) -> usize {
    let mut grid = map_to_number_grid(input);
    let mut total_removed = 0;

    loop {
        let indices = get_indices_for_removal(&grid);
        if indices.is_empty() {
            break;
        }
        remove_indices(&mut grid, &indices);
        total_removed += indices.len();
    }

    total_removed
}

fn remove_indices(grid: &mut [Vec<u8>], indices: &[(usize, usize)]) {
    for (x, y) in indices {
        grid[*y][*x] = 0;
    }
}

fn get_indices_for_removal(grid: &[Vec<u8>]) -> Vec<(usize, usize)> {
    let mut indices = Vec::new();

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 1 && check_surroundings(grid, x, y) < 4 {
                indices.push((x, y));
            }
        }
    }

    indices
}

fn check_surroundings(grid: &[Vec<u8>], x: usize, y: usize) -> u8 {
    let max_x = grid[0].len();
    let max_y = grid.len();
    let surrounding_indices = get_surrounding_indices(x, y, max_x, max_y);

    surrounding_indices
        .iter()
        .map(|(sx, sy)| grid[*sy][*sx])
        .sum()
}

const NEIGHBOR_OFFSETS: [(i32, i32); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn get_surrounding_indices(x: usize, y: usize, max_x: usize, max_y: usize) -> Vec<(usize, usize)> {
    NEIGHBOR_OFFSETS
        .iter()
        .filter_map(|(dx, dy)| {
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;
            if new_x >= 0 && new_x < max_x as i32 && new_y >= 0 && new_y < max_y as i32 {
                Some((new_x as usize, new_y as usize))
            } else {
                None
            }
        })
        .collect()
}

fn map_to_number_grid(input: &[String]) -> Vec<Vec<u8>> {
    input
        .iter()
        .map(|line| {
            line.trim()
                .as_bytes()
                .iter()
                .map(|&c| if c == b'@' { 1u8 } else { 0u8 })
                .collect::<Vec<u8>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn example_part_1() {
        let input = utils::read_lines(DAY, true);

        let result = solve_part1(&input);
        assert_eq!(result, 13);
    }

    #[test]
    #[ignore]
    fn solve_part_1() {
        let input_lines = utils::read_lines(DAY, false);

        let part1 = solve_part1(&input_lines);
        assert_eq!(part1, 1508);
    }

    #[test]
    #[ignore]
    fn example_part_2() {
        let input = utils::read_lines(DAY, true);
        let result = solve_part2(&input);
        assert_eq!(result, 43);
    }

    #[test]
    #[ignore]
    fn solve_part_2() {
        let input_lines = utils::read_lines(DAY, false);

        let part2 = solve_part2(&input_lines);
        assert_eq!(part2, 8538);
    }
}
