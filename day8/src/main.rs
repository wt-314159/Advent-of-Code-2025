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
    // for pair in closest_pairs.iter().take(10) {
    //     println!(
    //         "{:?} -> {:?} = {}",
    //         positions[pair.1], positions[pair.2], pair.0
    //     );
    // }
    let circuits = group_by_closest(&closest_pairs);
    for take_five in circuits.iter() {
        println!(
            "Circuit size: {}",
            take_five.as_ref().map_or(0, |v| v.len())
        );
    }
}

fn find_closest_pairs(positions: &[Position]) -> Vec<(f64, usize, usize)> {
    let len = positions.len();
    let mut pair_distances = Vec::with_capacity(len * (len - 1));
    for i in 0..len {
        for j in i + 1..len {
            let distance = positions[i].distance_from(&positions[j]);
            pair_distances.push((distance, i, j));
        }
    }
    pair_distances.sort_by(|a, b| a.0.total_cmp(&b.0));
    pair_distances
}

fn get_positions(input: &str) -> Vec<Position> {
    input.lines().map(Position::from).collect()
}

fn group_by_closest(closest_pairs: &[(f64, usize, usize)]) -> Vec<Option<HashSet<usize>>> {
    let mut circuits: Vec<Option<HashSet<usize>>> = Vec::new();
    let mut map: HashMap<usize, usize> = HashMap::new();
    for pair in closest_pairs {
        match (map.get(&pair.1), map.get(&pair.2)) {
            (None, None) => {
                let mut new_circuit = HashSet::new();
                new_circuit.insert(pair.1);
                new_circuit.insert(pair.2);
                circuits.push(Some(new_circuit));
                map.insert(pair.1, circuits.len() - 1);
                map.insert(pair.2, circuits.len() - 1);
            }
            (Some(&idx), None) => {
                let mut circuit = circuits[idx].take().expect("No circuit found!");
                circuit.insert(pair.2);
                circuits[idx] = Some(circuit);
                map.insert(pair.2, idx);
            }
            (None, Some(&idx)) => {
                let mut circuit = circuits[idx].take().expect("No circuit found");
                circuit.insert(pair.1);
                circuits[idx] = Some(circuit);
                map.insert(pair.1, idx);
            }
            (Some(idx1), Some(idx2)) if idx1 == idx2 => {
                // do nothing, already in the same circuit
            }
            (Some(&idx1), Some(&idx2)) => {
                // both junctions are already in separate circuits,
                // merge circuits into one
                let mut circuit;
                let (idx_use, idx_drop, pair_add);
                // eprintln!(
                //     "Indices: {}, {}; circuits.len(): {}; pair: {} -> {}",
                //     idx1,
                //     idx2,
                //     circuits.len(),
                //     pair.1,
                //     pair.2,
                // );
                let circuit1 = circuits[idx1].take().expect("No circuit at idx1");
                let circuit2 = circuits[idx2].take().expect("No circuit at idx2");
                if circuit1.len() > circuit2.len() {
                    circuit = circuit1;
                    (idx_use, idx_drop, pair_add) = (idx1, idx2, pair.2);
                } else {
                    circuit = circuit2;
                    (idx_use, idx_drop, pair_add) = (idx2, idx1, pair.1);
                }
                circuit.insert(pair_add);
                map.insert(pair_add, idx_use);
                // eprintln!("map[idx1]: {}, map[idx2]: {}", map[&pair.1], map[&pair.2]);
                circuits[idx_use] = Some(circuit);
                // eprintln!("Set circuits[{}] to {:?}", idx_use, circuits[idx_use]);
                // Also change all other references to dropped circuit to new circuit
                for (_, old_ref) in map.iter_mut().filter(|(_, c)| **c == idx_drop) {
                    *old_ref = idx_use;
                }
            }
        }
    }
    circuits.sort_by_key(|a| Reverse(a.as_ref().map_or(0, |v| v.len())));
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
