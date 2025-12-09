use std::{
    cmp::{max, min},
    collections::HashMap,
    time::Instant,
};

use crate::{types::point3d::Point2D, utils};

const DAY: u8 = 9;

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
    let points: Vec<Point2D> = input
        .iter()
        .filter_map(|line| Point2D::from_string(line))
        .collect();

    assert_eq!(points.len(), input.len());

    create_square_list(&points).last().unwrap().0
}

fn solve_part2(input: &[String]) -> usize {
    let mut points: Vec<Point2D> = input
        .iter()
        .filter_map(|line| Point2D::from_string(line))
        .collect();
    assert_eq!(points.len(), input.len());

    // Allow circle by adding first point to end
    points.push(points[0]);

    let mut collision_map: HashMap<(isize, isize), bool> = HashMap::new();

    for i in 0..(points.len() - 1) {
        let start = &points[i];
        let end = &points[i + 1];

        Point2D::points_between(start, end).iter().for_each(|p| {
            collision_map.insert((p.x, p.y), true);
        });
    }

    let mut square_list = create_square_list(&points);

    let len = square_list.len();
    square_list.reverse();
    square_list = square_list.into_iter().skip(len / 4).collect();

    for (dist, i, j) in square_list.into_iter() {
        let point1 = &points[i];
        let point2 = &points[j];

        let mut mistake = points.iter().any(|p| p.inside_rectangle(point1, point2));
        if !mistake {
            for border_point in get_inner_border_points(*point1, *point2) {
                if collision_map.contains_key(&(border_point.x, border_point.y)) {
                    mistake = true;
                    break;
                }
            }
        }
        if !mistake {
            return dist;
        }
    }

    0
}

fn create_square_list(points: &[Point2D]) -> Vec<(usize, usize, usize)> {
    let mut distances = Vec::new();

    for (i, point1) in points.iter().enumerate() {
        for (j, point2) in points.iter().enumerate().skip(i + 1) {
            let dist = Point2D::square_between(point1, point2);
            distances.push((dist as usize, i, j));
        }
    }

    distances.sort_by(|a, b| a.0.cmp(&b.0));
    distances
}

fn get_inner_border_points(point1: Point2D, point2: Point2D) -> Vec<Point2D> {
    let min_x = min(point1.x, point2.x);
    let max_x = max(point1.x, point2.x);
    let min_y = min(point1.y, point2.y);
    let max_y = max(point1.y, point2.y);

    let mut border_points = Vec::new();

    for x in (min_x + 1)..max_x {
        border_points.push(Point2D { x, y: min_y + 1 });
        border_points.push(Point2D { x, y: max_y - 1 });
    }

    for y in (min_y + 1)..max_y {
        border_points.push(Point2D { x: min_x + 1, y });
        border_points.push(Point2D { x: max_x - 1, y });
    }

    border_points
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore]
    fn example_part_1() {
        let input = utils::read_lines(DAY, true);

        let result = solve_part1(&input);
        assert_eq!(result, 50);
    }

    #[test]
    #[ignore]
    fn solve_part_1() {
        let input_lines = utils::read_lines(DAY, false);

        let part1 = solve_part1(&input_lines);
        assert_eq!(part1, 4744899849);
    }

    #[test]
    #[ignore]
    fn example_part_2() {
        let input = utils::read_lines(DAY, true);
        let result = solve_part2(&input);
        assert_eq!(result, 24);
    }

    #[test]
    #[ignore]
    fn solve_part_2() {
        let input_lines = utils::read_lines(DAY, false);

        let part2 = solve_part2(&input_lines);
        assert_eq!(part2, 1540192500);
    }
}
