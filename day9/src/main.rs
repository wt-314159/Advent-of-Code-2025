fn main() {
    let input = include_str!("../puzzle_input.txt");
    let test_input = r"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    let positions = parse_positions(input);
    let max_area = find_largest_rect(&positions);
    println!("Largest rectangle: {max_area}");
}

fn parse_positions(input: &str) -> Vec<Position> {
    input.lines().map(Position::from).collect()
}

fn find_largest_rect(positions: &[Position]) -> usize {
    let mut max_size = 0;
    for i in 0..positions.len() {
        for j in i..positions.len() {
            let (pos1, pos2) = (positions[i], positions[j]);
            let area = pos1.find_rect_area(&pos2);
            if area > max_size {
                max_size = area;
            }
        }
    }
    max_size
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Position {
    x: usize,
    y: usize,
}

impl From<&str> for Position {
    fn from(value: &str) -> Self {
        let (x, y) = value.split_once(',').expect("No ',' in line!");
        let (x, y) = (
            x.parse().expect("Failed to parse x"),
            y.parse().expect("Failed to parse y"),
        );
        Position { x, y }
    }
}

impl Position {
    fn find_rect_area(&self, other: &Position) -> usize {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }
}
