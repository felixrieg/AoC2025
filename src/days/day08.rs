use std::time::Instant;

use crate::{
    types::{point3d::Point3D, union_find::UnionFind},
    utils,
};

const DAY: u8 = 8;

pub fn solve() {
    let inputs = utils::read_lines(DAY, false);

    let start = Instant::now();
    let part1 = solve_part1(&inputs, 1000);
    let duration1 = start.elapsed();

    let start = Instant::now();
    let part2 = solve_part2(&inputs);
    let duration2 = start.elapsed();

    utils::print_grid(DAY, part1, part2, duration1, duration2);
}

fn solve_part1(input: &[String], take: usize) -> usize {
    let points = parse_points(input);
    let distances = generate_distance_list(&points);

    let mut uf = UnionFind::new(points.len());

    for (_, idx1, idx2) in distances.iter().take(take) {
        uf.union(*idx1, *idx2);
    }

    let mut sizes: Vec<_> = uf.cluster_sizes().values().copied().collect();
    sizes.sort_by(|a, b| b.cmp(a));

    sizes.iter().take(3).product()
}

fn solve_part2(input: &[String]) -> usize {
    let points = parse_points(input);
    let distances = generate_distance_list(&points);

    let mut uf = UnionFind::new(points.len());
    let mut result = (0, 0);

    for (_, idx1, idx2) in distances {
        uf.union(idx1, idx2);

        let sizes = uf.cluster_sizes();
        if sizes.len() == 1 {
            result = (idx1, idx2);
            break;
        }
    }

    (points[result.0].x * points[result.1].x).unsigned_abs()
}

fn parse_points(input: &[String]) -> Vec<Point3D> {
    let points: Vec<Point3D> = input
        .iter()
        .filter_map(|line| Point3D::from_string(line))
        .collect();

    assert_eq!(points.len(), input.len());
    points
}

fn generate_distance_list(points: &[Point3D]) -> Vec<(f64, usize, usize)> {
    let mut distances: Vec<_> = (0..points.len())
        .flat_map(|i| {
            (i + 1..points.len()).map(move |j| {
                let dist = points[i].distance_to(&points[j]);
                (dist, i, j)
            })
        })
        .collect();

    distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    distances
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn example_part_1() {
        let input = utils::read_lines(DAY, true);

        let result = solve_part1(&input, 10);
        assert_eq!(result, 40);
    }

    #[test]
    #[ignore]
    fn solve_part_1() {
        let input_lines = utils::read_lines(DAY, false);

        let part1 = solve_part1(&input_lines, 1000);
        assert_eq!(part1, 66912);
    }

    #[test]
    #[ignore]
    fn example_part_2() {
        let input = utils::read_lines(DAY, true);
        let result = solve_part2(&input);
        assert_eq!(result, 25272);
    }

    #[test]
    #[ignore]
    fn solve_part_2() {
        let input_lines = utils::read_lines(DAY, false);

        let part2 = solve_part2(&input_lines);
        assert_eq!(part2, 724454082);
    }
}
