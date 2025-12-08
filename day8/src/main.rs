use std::collections::BTreeMap;

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
    for pair in closest_pairs.iter().take(10) {
        println!(
            "{:?} -> {:?} = {}",
            positions[pair.1.0], positions[pair.1.1], pair.0
        );
    }
}

fn find_closest_pairs(positions: &[Position]) -> BTreeMap<usize, (usize, usize)> {
    let mut map = BTreeMap::new();
    for i in 0..positions.len() {
        for j in i + 1..positions.len() {
            let distance = positions[i].distance_from(&positions[j]);
            map.insert(distance, (i, j));
        }
    }
    map
}

fn get_positions(input: &str) -> Vec<Position> {
    input.lines().map(Position::from).collect()
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Position {
    x: usize,
    y: usize,
    z: usize,
}

impl Position {
    fn distance_from(&self, other: &Self) -> usize {
        // Probably don't need the exact distance here,
        // just the sums of the offsets in each of the
        // 3 axes should be fine for our purposes.
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y) + self.z.abs_diff(other.z)
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
