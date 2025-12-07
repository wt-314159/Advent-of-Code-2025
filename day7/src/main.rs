fn main() {
    let input = include_str!("../puzzle_input.txt");

    let test_input = r".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    let split_count = count_splits(input);
    println!("Split count: {split_count}");
}

fn count_splits(input: &str) -> usize {
    let mut total = 0;
    let mut input_lines = input.lines().peekable();
    let first_line = input_lines.peek().expect("No lines!");
    let mut beams = vec![false; first_line.len()];
    let beam_index = first_line.find('S').expect("No 'S' on first line");
    beams[beam_index] = true;

    // We can skip every other line, and start on 3rd line
    for line in input_lines.step_by(2).skip(1) {
        for (i, c) in line.chars().enumerate() {
            // If an existing beam hits a splitter...
            if c == '^' && beams[i] {
                // ...remove beam at splitter, add one either side, increment total
                beams[i] = false;
                beams[i - 1] = true;
                beams[i + 1] = true;
                total += 1;
            }
        }
    }
    total
}
