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

    // part_one(input);
    part_two(input);
}

#[allow(dead_code)]
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
    eprintln!("Num positions: {}", positions.len());
    // use 0 in find_n_closest_pairs to get all pairs
    let pairs_by_distance = find_n_closest_pairs(&positions, 0);
    eprintln!("Num pairs: {}", pairs_by_distance.len());
    let last_pair = group_all(&pairs_by_distance, positions.len());
    println!(
        "Last pair: {:?} & {:?}",
        positions[last_pair.0], positions[last_pair.1]
    );
    let (x1, x2) = (positions[last_pair.0].x, positions[last_pair.1].x);
    println!("{} * {} = {}", x1, x2, x1 * x2);
}

fn find_n_closest_pairs(positions: &[Position], n: usize) -> Vec<(f64, usize, usize)> {
    let len = positions.len();
    let mut pair_distances = Vec::with_capacity((len * (len - 1)) / 2);
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
        eprintln!("Truncated to {}", n);
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

fn group_all(closest_pairs: &[(f64, usize, usize)], num_junctions: usize) -> (usize, usize) {
    let mut sets = Vec::new();
    let mut idx_map = HashMap::new();
    for pair in closest_pairs {
        match (idx_map.get(&pair.1), idx_map.get(&pair.2)) {
            (None, None) => {
                let mut new_map = HashSet::new();
                new_map.insert(pair.1);
                new_map.insert(pair.2);
                idx_map.insert(pair.1, sets.len());
                idx_map.insert(pair.2, sets.len());
                sets.push(Some(new_map));
            }
            (Some(&idx1), None) => {
                sets[idx1].as_mut().unwrap().insert(pair.2);
                idx_map.insert(pair.2, idx1);
                if sets[idx1].as_ref().unwrap().len() == num_junctions {
                    return (pair.1, pair.2);
                }
            }
            (None, Some(&idx2)) => {
                sets[idx2].as_mut().unwrap().insert(pair.1);
                idx_map.insert(pair.1, idx2);
                if sets[idx2].as_ref().unwrap().len() == num_junctions {
                    return (pair.1, pair.2);
                }
            }
            (Some(idx1), Some(idx2)) if idx1 == idx2 => {
                // Do nothing, already in same circuit
            }
            (Some(&idx1), Some(&idx2)) => {
                // Check if lengths of both sets sum to num of junctions,
                // if so, this will be the final merge to add all juctions
                // to one circuit, so just break early and return the pair
                let (len1, len2) = (
                    sets[idx1].as_ref().unwrap().len(),
                    sets[idx2].as_ref().unwrap().len(),
                );
                if len1 + len2 == num_junctions {
                    return (pair.1, pair.2);
                }
                // Now we need to merge sets, and make sure idx_map is updated
                let (set_add, set_drop, idx_add);
                if len1 >= len2 {
                    set_drop = sets[idx2].take().unwrap();
                    set_add = sets[idx1].as_mut().unwrap();
                    idx_add = idx1;
                } else {
                    set_drop = sets[idx1].take().unwrap();
                    set_add = sets[idx2].as_mut().unwrap();
                    idx_add = idx2;
                }
                for junc in set_drop {
                    set_add.insert(junc);
                    idx_map.insert(junc, idx_add);
                }
                if set_add.len() == num_junctions {}
            }
        }
    }
    // Should hopefully never reach here, as after iterating
    // through all connections, we should have one circuit
    panic!("Failed to merge all junctions into one circuit");
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
