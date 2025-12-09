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

    let positions = get_positions(test_input);
    let closest_pairs = find_closest_pairs(&positions);
    for pair in closest_pairs.iter() {
        println!(
            "{:?} -> {:?} = {}, simple dist. {}",
            positions[pair.1],
            positions[pair.2],
            pair.0,
            dumb_distance(&positions[pair.1], &positions[pair.2])
        );
    }
    let circuits = group_by_closest(&closest_pairs, 10);
    for take_five in circuits.iter() {
        println!("Circuit size: {}", take_five.len());
        // for position in take_five.as_ref().unwrap() {
        //     println!("\t{:?}", position);
        // }
    }
}

fn dumb_distance(pos1: &Position, pos2: &Position) -> f64 {
    (pos1.x - pos2.x).abs() + (pos1.y - pos2.y).abs() + (pos1.z - pos2.z).abs()
}

fn find_closest_pairs(positions: &[Position]) -> Vec<(f64, usize, usize)> {
    let len = positions.len();
    let mut used = HashSet::new();
    let mut pair_distances = Vec::with_capacity(len);
    for i in 0..len {
        // we've already found closest pair for this idx
        if used.contains(&i) {
            continue;
        }
        let (min_idx, min) = positions
            .iter()
            .enumerate()
            .filter(|(idx, _)| *idx != i)
            .map(|(idx, pos)| (idx, pos.distance_from(&positions[i])))
            .min_by(|a, b| a.1.total_cmp(&b.1))
            .expect("Failed to find minimum");
        pair_distances.push((min, i, min_idx));
        used.insert(i);
        used.insert(min_idx);
    }
    pair_distances.sort_by(|a, b| a.0.total_cmp(&b.0));
    pair_distances
}

fn get_positions(input: &str) -> Vec<Position> {
    input.lines().map(Position::from).collect()
}

fn group_by_closest_two(closest_pairs: &[(f64, usize, usize)], num_connections: usize) {}

fn group_by_closest(
    closest_pairs: &[(f64, usize, usize)],
    num_connections: usize,
) -> Vec<HashSet<usize>> {
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
        if connection_count >= num_connections {
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
                if connection_count >= num_connections {
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
