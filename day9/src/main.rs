use std::cmp::Reverse;

fn main() {
    #[allow(unused_variables)]
    let input = include_str!("../puzzle_input.txt");
    #[allow(unused_variables)]
    let test_input = r"7,1
11,1
11,4
11,7
9,7
9,5
2,5
2,3
7,3";

    // part_one(input);
    part_two(input);
}

#[allow(dead_code)]
fn part_one(input: &str) {
    let positions = parse_positions(input);
    let max_area = find_largest_rect(&positions);
    println!("Largest rectangle: {max_area}");
}

fn part_two(input: &str) {
    let positions = parse_positions(input);
    let lines = find_lines(&positions);
    let lines = consolidate_lines(lines);
    // print_tiles(&positions, &lines);
    // println!("{lines:#?}");
    let max_rect = find_largest_green_rect(&positions, &lines);
    println!("Largest green rectangle: {}", max_rect.2);
    println!("From {:?} to {:?}", max_rect.0, max_rect.1);
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

fn find_largest_green_rect(positions: &[Position], lines: &[Line]) -> (Position, Position, usize) {
    let mut rectangles = Vec::new();
    for i in 0..positions.len() {
        for j in i..positions.len() {
            let (pos1, pos2) = (positions[i], positions[j]);
            let area = pos1.find_rect_area(&pos2);
            rectangles.push((pos1, pos2, area));
        }
    }
    rectangles.sort_by_key(|r| Reverse(r.2));

    *rectangles
        .iter()
        .find(|r| is_rect_all_green(r, positions, lines))
        .expect("No rectangles are all green")
}

fn is_rect_all_green(
    rectangle: &(Position, Position, usize),
    positions: &[Position],
    lines: &[Line],
) -> bool {
    // Check if there are any points inside the rectangle, ignoring a 1 width border
    // Should only be possible for the rect to be all green (all inside the shape) if
    // there are no points inside it
    let (min_x, max_x) = min_max(rectangle.0.x, rectangle.1.x);
    let (min_y, max_y) = min_max(rectangle.0.y, rectangle.1.y);
    if positions
        .iter()
        .any(|p| p.x > min_x && p.x < max_x && p.y > min_y && p.y < max_y)
    {
        return false;
    }
    // Also need to check if any lines cross through the center of the space
    !lines
        .iter()
        .any(|l| l.crosses_rect(min_x, max_x, min_y, max_y))
}

fn find_lines(positions: &[Position]) -> Vec<Line> {
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

fn consolidate_lines(lines: Vec<Line>) -> Vec<Line> {
    let mut new_lines = Vec::with_capacity(lines.len());
    let mut iter = lines.iter();
    let mut prev_line = *iter.next().unwrap();
    for cur_line in iter {
        if prev_line.is_end_to_end(cur_line) {
            prev_line.end = cur_line.end;
        } else {
            new_lines.push(prev_line);
            prev_line = *cur_line;
        }
    }
    // We always end the loop with a prev_line still to push
    new_lines.push(prev_line);
    new_lines
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

    fn is_end_to_end(&self, other: &Line) -> bool {
        // all points on same x, or all points on same y
        (self.start.x == self.end.x && other.start.x == self.start.x && other.end.x == self.start.x)
            || (self.start.y == self.end.y
                && other.start.y == self.start.y
                && other.end.y == self.start.y)
    }

    fn crosses_rect(&self, min_x: usize, max_x: usize, min_y: usize, max_y: usize) -> bool {
        // horizontal lines
        if self.start.y == self.end.y {
            // first check line is on a row between min_x and max_x
            self.start.y > min_y
                && self.start.y < max_y
                // Then check line's start and end aren't either both
                // before or both after min_x and max_x respectively
                && !((self.start.x <= min_x && self.end.x <= min_x)
                    || (self.start.x >= max_x && self.end.x >= max_x))
        } else {
            // vertical lines
            self.start.x > min_x
                && self.start.x < max_x
                && !((self.start.y <= min_y && self.end.y <= min_y)
                    || (self.start.y >= max_y && self.end.y >= max_y))
        }
    }
}

#[allow(dead_code)]
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
