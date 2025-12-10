use std::time::Instant;

use microlp::{ComparisonOp, OptimizationDirection, Problem, Variable};

use crate::utils;

const DAY: u8 = 10;

struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

impl Machine {
    fn from_str(input: &str) -> Self {
        let s: Vec<&str> = input.split(" ").collect();

        let lights_str = if let Some(content) = utils::find_between(s[0], '[', ']') {
            content
        } else {
            panic!("Invalid machine input: no lights found");
        };

        let lights: Vec<bool> = lights_str.chars().map(|c| c == '#').collect();

        let buttons: Vec<Vec<usize>> = s
            .iter()
            .skip(1)
            .rev()
            .skip(1)
            .filter_map(|token| utils::find_between(token, '(', ')').map(utils::parse_numbers))
            .collect();

        let joltage = if let Some(content) = utils::find_between(s.last().unwrap(), '{', '}') {
            utils::parse_numbers(content)
        } else {
            panic!("Invalid machine input: no joltage found");
        };

        Machine {
            lights,
            buttons,
            joltage,
        }
    }
}

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
    let machines: Vec<Machine> = input.iter().map(|line| Machine::from_str(line)).collect();

    machines
        .iter()
        .map(|machine| {
            let mut state = vec![false; machine.lights.len()];
            if let Some((button_sequence, _cost)) = traverse_machine(machine, &mut state, 0) {
                button_sequence.len()
            } else {
                println!("No solution found for machine.");
                0
            }
        })
        .sum()
}

fn press(state: &mut [bool], button: &Vec<usize>) {
    for &idx in button {
        if idx < state.len() {
            state[idx] = !state[idx];
        }
    }
}

fn traverse_machine(
    machine: &Machine,
    state: &mut Vec<bool>,
    button: usize,
) -> Option<(Vec<usize>, usize)> {
    if machine.lights == *state {
        return Some((vec![], 0));
    } else if button >= machine.buttons.len() {
        return None;
    }

    // First check if not pressed any button leads to solution
    let result_no_press = traverse_machine(machine, state, button + 1);

    // Then try pressing current button
    press(state, &machine.buttons[button]);
    let result_press = traverse_machine(machine, state, button + 1);
    press(state, &machine.buttons[button]); // backtrack

    match (result_no_press, result_press) {
        (Some((seq, cost)), Some((mut seq2, cost2))) => {
            if cost <= cost2 {
                Some((seq, cost))
            } else {
                seq2.insert(0, button);
                Some((seq2, cost2 + 1))
            }
        }
        (Some((seq, cost)), None) => Some((seq, cost)),
        (None, Some((mut seq2, cost2))) => {
            seq2.insert(0, button);
            Some((seq2, cost2 + 1))
        }
        (None, None) => None,
    }
}

fn solve_part2(input: &[String]) -> usize {
    let machines: Vec<Machine> = input.iter().map(|line| Machine::from_str(line)).collect();

    machines.iter().map(create_solution).sum()
}

fn create_solution(machine: &Machine) -> usize {
    let mut problem = Problem::new(OptimizationDirection::Minimize);
    let variables = (0..machine.buttons.len())
        .map(|_| problem.add_integer_var(1.0, (0, i32::MAX)))
        .collect::<Vec<_>>();

    for (joltage_index, joltage) in machine.joltage.iter().enumerate() {
        let expr: Vec<(Variable, f64)> = machine
            .buttons
            .iter()
            .enumerate()
            .filter_map(|(button_index, btn)| {
                if btn.contains(&joltage_index) {
                    Some((variables[button_index], 1.0))
                } else {
                    None
                }
            })
            .collect();
        problem.add_constraint(&expr, ComparisonOp::Eq, *joltage as f64);
    }
    problem.solve().unwrap().objective().round() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn example_part_1() {
        let input = utils::read_lines(DAY, true);

        let result = solve_part1(&input);
        assert_eq!(result, 7);
    }

    #[test]
    #[ignore]
    fn solve_part_1() {
        let input_lines = utils::read_lines(DAY, false);

        let part1 = solve_part1(&input_lines);
        assert_eq!(part1, 486);
    }

    #[test]
    #[ignore]
    fn example_part_2() {
        let input = utils::read_lines(DAY, true);
        let result = solve_part2(&input);
        assert_eq!(result, 33);
    }

    #[test]
    #[ignore]
    fn solve_part_2() {
        let input_lines = utils::read_lines(DAY, false);

        let part2 = solve_part2(&input_lines);
        assert_eq!(part2, 17820);
    }
}
