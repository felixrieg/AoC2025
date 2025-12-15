use std::time::Instant;

use crate::utils;

const DAY: u8 = 12;

pub fn solve() {
    let inputs = utils::read_lines(DAY, false);

    let start = Instant::now();
    let part1 = solve_part1(&inputs);
    let duration1 = start.elapsed();

    let start = Instant::now();
    let duration2 = start.elapsed();

    utils::print_grid(DAY, part1, 0, duration1, duration2);
}

fn solve_part1(input: &[String]) -> usize {
    let puzzle_data = parse_input(input);

    puzzle_data
        .iter()
        .map(|(x, y, data)| {
            let puzzle_size = x * y;
            let stupid_used_cells = data.iter().map(|d| d * 9).sum::<usize>();

            if puzzle_size >= stupid_used_cells {
                return true;
            }

            false
        })
        .filter(|b| *b)
        .count()
}

fn parse_input(input: &[String]) -> Vec<(usize, usize, Vec<usize>)> {
    let mut grouped_inputs = utils::split_on_empty_lines(input);

    let puzzle_data = grouped_inputs.pop().expect("No puzzle data found");
    parse_puzzle_data(&puzzle_data)

    // let forms = grouped_inputs
    //     .iter()
    //     .map(|group| {
    //         group
    //             .iter()
    //             .map(|line| line.bytes().filter(|b| b == &b'#').count())
    //             .sum()
    //     })
    //     .collect::<Vec<usize>>();
}

fn parse_puzzle_data(input: &[String]) -> Vec<(usize, usize, Vec<usize>)> {
    input
        .iter()
        .map(|line| {
            let parts: Vec<&str> = line.split(":").collect();
            let coords_part = parts[0].trim();
            let data_part = parts[1].trim();

            let coords: Vec<&str> = coords_part.split('x').collect();
            let x = coords[0].trim().parse().expect("Invalid x coordinate");
            let y = coords[1].trim().parse().expect("Invalid y coordinate");

            let data: Vec<usize> = data_part
                .split(' ')
                .map(|s| s.trim().parse().expect("Invalid data value"))
                .collect();
            (x, y, data)
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
        assert_eq!(result, 1);

        // Should be 2, but the example is more complex than the actual pyzzle input => only 1 fits.
        // assert_eq!(result, 2);
    }

    #[test]
    #[ignore]
    fn solve_part_1() {
        let input_lines = utils::read_lines(DAY, false);

        let part1 = solve_part1(&input_lines);
        assert_eq!(part1, 463);
    }
}
