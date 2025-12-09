use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
};

fn main() {
    let input = include_str!("../puzzle_input.txt");

    #[allow(unused_variables)]
    let test_input = r"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    part_one(input);
}

fn part_one(input: &str) {
    let positions = get_positions(input);
    let num_closest_connections = 1000;
    let closest_pairs = find_n_closest_pairs(&positions, num_closest_connections);

    let circuits = group_by_closest(&closest_pairs);
    for take_five in circuits.iter().take(5) {
        println!("Circuit size: {}", take_five.len());
    }
    let mult = circuits
        .iter()
        .take(3)
        .fold(1, |i, circuit| i * circuit.len());
    println!(
        "Top 3 multiplied: {} * {} * {} = {}",
        circuits[0].len(),
        circuits[1].len(),
        circuits[2].len(),
        mult
    );
}

fn part_two(input: &str) {
    let positions = get_positions(input);
    // use 0 in find_n_closest_pairs to get all pairs
    let pairs_by_distance = find_n_closest_pairs(&positions, 0);
}

fn find_n_closest_pairs(positions: &[Position], n: usize) -> Vec<(f64, usize, usize)> {
    let len = positions.len();
    let mut pair_distances = Vec::with_capacity(len);
    for i in 0..len {
        for j in i + 1..len {
            let distance = positions[i].distance_from(&positions[j]);
            pair_distances.push((distance, i, j));
        }
    }
    pair_distances.sort_by(|a, b| a.0.total_cmp(&b.0));
    // pass n = 0 to return all distances
    if n != 0 {
        pair_distances.truncate(n);
    }
    pair_distances
}

fn get_positions(input: &str) -> Vec<Position> {
    input.lines().map(Position::from).collect()
}

// This approach goes from one connection to the next, grouping,
// until there are no more connections, to avoid having to join
// 2 existing circuits together at any point. Unfortunately, we
// can't use this approach for part 2, since we need to join all
// junctions into one big circuit, and see which is the last
// connection to be made before they're all linked.
fn group_by_closest(closest_pairs: &[(f64, usize, usize)]) -> Vec<HashSet<usize>> {
    let mut circuits = Vec::new();
    let mut used = HashSet::new();
    let mut connection_count = 0;
    'outer: while let Some(start_pair) = closest_pairs.iter().find(|p| !used.contains(&p.1)) {
        let mut circuit = HashSet::new();
        circuit.insert(start_pair.1);
        circuit.insert(start_pair.2);
        used.insert(start_pair.1);
        used.insert(start_pair.2);
        connection_count += 1;
        if connection_count >= closest_pairs.len() {
            break;
        }
        loop {
            let mut added = 0;
            for pair in closest_pairs {
                if circuit.contains(&pair.1) && !circuit.contains(&pair.2) {
                    circuit.insert(pair.2);
                    used.insert(pair.2);
                    added += 1;
                    connection_count += 1;
                } else if circuit.contains(&pair.2) && !circuit.contains(&pair.1) {
                    circuit.insert(pair.1);
                    used.insert(pair.1);
                    added += 1;
                    connection_count += 1;
                }
                if connection_count >= closest_pairs.len() {
                    break 'outer;
                }
            }
            if added == 0 {
                break;
            }
        }
        circuits.push(circuit);
    }
    circuits.sort_by_key(|c| Reverse(c.len()));
    circuits
}

fn group_all(closest_pairs: &[(f64, usize, usize)]) {}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Position {
    x: f64,
    y: f64,
    z: f64,
}

impl Position {
    fn distance_from(&self, other: &Self) -> f64 {
        let (x_diff, y_diff, z_diff) = (self.x - other.x, self.y - other.y, self.z - other.z);
        let (x_diff, y_diff, z_diff) = (x_diff * x_diff, y_diff * y_diff, z_diff * z_diff);
        (x_diff + y_diff + z_diff).sqrt()
    }
}

impl From<&str> for Position {
    fn from(value: &str) -> Self {
        let split: Vec<&str> = value.split(',').collect();
        let (x, y, z) = (split[0], split[1], split[2]);
        let (x, y, z) = (x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap());
        Position { x, y, z }
    }
}
