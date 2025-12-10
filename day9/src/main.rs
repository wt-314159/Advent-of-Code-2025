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

    // part_one(input);
    part_two(test_input);
}

#[allow(dead_code)]
fn part_one(input: &str) {
    let positions = parse_positions(input);
    let max_area = find_largest_rect(&positions);
    println!("Largest rectangle: {max_area}");
}

fn part_two(input: &str) {
    let positions = parse_positions(input);
    let lines = find_green_tiles(&positions);
    print_tiles(&positions, &lines);
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

fn find_largest_green_rect(positions: &[Position]) -> usize {
    todo!()
}

fn find_green_tiles(positions: &[Position]) -> Vec<Line> {
    let mut lines = Vec::new();
    let mut iter = positions.iter().peekable();
    while let Some(start) = iter.next()
        && let Some(end) = iter.peek()
    {
        lines.push(Line::new(*start, **end));
    }
    // join last and first line
    lines.push(Line::new(
        *positions.last().unwrap(),
        *positions.first().unwrap(),
    ));
    lines
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

#[derive(Clone, Copy, Debug, PartialEq)]
struct Line {
    start: Position,
    end: Position,
}

impl Line {
    fn new(start: Position, end: Position) -> Line {
        Line { start, end }
    }
}

fn print_tiles(positions: &[Position], lines: &[Line]) {
    let max_x = positions
        .iter()
        .max_by_key(|p| p.x)
        .expect("No positions")
        .x
        + 3;
    let max_y = positions
        .iter()
        .max_by_key(|p| p.y)
        .expect("No positions")
        .y
        + 2;
    let mut grid: Vec<Vec<char>> = (0..max_y).map(|_| vec!['.'; max_x]).collect();
    for pos in positions {
        grid[pos.y][pos.x] = '#';
    }
    for line in lines {
        if line.start.x == line.end.x {
            let (min_y, max_y) = min_max(line.start.y, line.end.y);
            for row in grid[min_y + 1..max_y].iter_mut() {
                if row[line.start.x] == '.' {
                    row[line.start.x] = 'X';
                }
            }
        } else {
            let (min_x, max_x) = min_max(line.start.x, line.end.x);
            for c in &mut grid[line.start.y][min_x + 1..max_x] {
                if *c == '.' {
                    *c = 'X';
                }
            }
        }
    }

    for row in grid {
        for col in row {
            print!("{col}");
        }
        println!();
    }
}

fn min_max(a: usize, b: usize) -> (usize, usize) {
    if a < b { (a, b) } else { (b, a) }
}
