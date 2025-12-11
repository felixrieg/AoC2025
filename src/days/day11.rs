use std::{collections::VecDeque, time::Instant};

use crate::utils;

const DAY: u8 = 11;

pub fn solve() {
    let inputs = utils::read_lines(DAY, false);

    let start = Instant::now();
    let part1 = solve_part1(&inputs, "you");
    let duration1 = start.elapsed();

    let start = Instant::now();
    let part2 = solve_part2(&inputs);
    let duration2 = start.elapsed();

    utils::print_grid(DAY, part1, part2, duration1, duration2);
}

fn create_variables(input: &[String], start: &str) -> (Vec<String>, usize) {
    let mut variables: Vec<String> = input
        .iter()
        .map(|line| line.split(":").collect::<Vec<&str>>()[0].to_string())
        .collect();
    variables.insert(0, "out".to_string());

    let start_index = variables
        .iter()
        .position(|var| var == start)
        .expect("Start variable not found");

    (variables, start_index)
}

fn make_mapping(variables: &[String], input: &[String]) -> Vec<Vec<usize>> {
    let mut mapping: Vec<Vec<usize>> = vec![vec![]; variables.len()];

    for line in input {
        let split = line.split(":").collect::<Vec<&str>>();
        assert!(split.len() == 2, "Invalid input line: {}", line);
        let var_index = variables
            .iter()
            .position(|v| v == split[0])
            .expect("Variable not found");
        mapping[var_index] = split[1]
            .trim()
            .split(" ")
            .map(|dest| {
                variables
                    .iter()
                    .position(|v| v == dest)
                    .expect("Variable not found")
            })
            .collect();
    }
    mapping[0] = vec![0];
    mapping
}

fn traverse(mapping: &Vec<Vec<usize>>, current: usize) -> usize {
    if current == 0 {
        return 1;
    }

    mapping[current]
        .iter()
        .map(|&next| traverse(mapping, next))
        .sum()
}

fn solve_part1(input: &[String], start: &str) -> usize {
    let (variables, start_index) = create_variables(input, start);
    let mapping = make_mapping(&variables, input);
    traverse(&mapping, start_index)
}

fn reverse_mapping(mapping: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut rev_map: Vec<Vec<usize>> = vec![vec![]; mapping.len()];

    for (src, destination) in mapping.iter().enumerate() {
        for &dest in destination {
            rev_map[dest].push(src);
        }
    }

    rev_map
}

fn merge_paths(paths: &mut Vec<Path>) {
    let mut merged: Vec<Path> = vec![];
    for path in paths.iter() {
        if let Some(existing) = merged
            .iter_mut()
            .find(|p| p.fft_seen == path.fft_seen && p.dac_seen == path.dac_seen)
        {
            existing.options += path.options;
        } else {
            merged.push(path.clone());
        }
    }
    *paths = merged;
}

#[derive(Debug, Clone)]
struct Path {
    pub options: usize,
    pub fft_seen: bool,
    pub dac_seen: bool,
}

fn solve_part2(input: &[String]) -> usize {
    let (variables, start_index) = create_variables(input, "svr");
    let mapping = make_mapping(&variables, input);
    let rev_mapping = reverse_mapping(&mapping);

    let mut solutions: Vec<Vec<Path>> = vec![vec![]; variables.len()];
    solutions[0].push(Path {
        options: 1,
        fft_seen: false,
        dac_seen: false,
    });

    let fft_variable = variables
        .iter()
        .position(|var| var == "fft")
        .expect("Start variable not found");

    let dac_variable = variables
        .iter()
        .position(|var| var == "dac")
        .expect("Start variable not found");

    let mut queue: VecDeque<usize> = VecDeque::from([0]);

    loop {
        if queue.is_empty() {
            break;
        }

        let current_element = queue.pop_front().unwrap();

        let mut every_path_done: bool = true;
        let mut traversed_paths: Vec<Path> = vec![];
        for &next in &mapping[current_element] {
            if solutions[next].is_empty() {
                every_path_done = false;
                queue.push_back(next);
            } else if !every_path_done {
                continue;
            } else {
                for path in solutions[next].clone() {
                    traversed_paths.push(path);
                }
            }
        }

        if !every_path_done {
            queue.push_back(current_element);
            continue;
        }

        if current_element == fft_variable || current_element == dac_variable {
            traversed_paths = traversed_paths
                .iter()
                .map(|path| Path {
                    options: path.options,
                    fft_seen: if current_element == fft_variable {
                        true
                    } else {
                        path.fft_seen
                    },
                    dac_seen: if current_element == dac_variable {
                        true
                    } else {
                        path.dac_seen
                    },
                })
                .collect();
        }
        merge_paths(&mut traversed_paths);
        solutions[current_element] = traversed_paths;
        if current_element == start_index {
            break;
        }
        for &prev in &rev_mapping[current_element] {
            if !queue.contains(&prev) && prev != 0 {
                queue.push_back(prev);
            }
        }
    }

    if solutions[start_index].is_empty() {
        panic!("No solution found");
    }
    solutions[start_index]
        .iter()
        .filter(|path| path.fft_seen && path.dac_seen)
        .map(|path| path.options)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn example_part_1() {
        let input = utils::read_lines(DAY, true);

        let result = solve_part1(&input, "svr");
        assert_eq!(result, 8);
    }

    #[test]
    #[ignore]
    fn solve_part_1() {
        let input_lines = utils::read_lines(DAY, false);

        let part1 = solve_part1(&input_lines, "you");
        assert_eq!(part1, 566);
    }

    #[test]
    #[ignore]
    fn example_part_2() {
        let input = utils::read_lines(DAY, true);
        let result = solve_part2(&input);
        assert_eq!(result, 2);
    }

    #[test]
    #[ignore]
    fn solve_part_2() {
        let input_lines = utils::read_lines(DAY, false);

        let part2 = solve_part2(&input_lines);
        assert_eq!(part2, 331837854931968);
    }
}
