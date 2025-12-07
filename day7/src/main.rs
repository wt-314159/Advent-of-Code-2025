fn main() {
    let input = include_str!("../puzzle_input.txt");

    #[allow(unused_variables)]
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

    // let split_count = count_splits(input);
    // println!("Split count: {split_count}");
    let paths_count = count_pathways(input);
    println!("Number of paths: {paths_count}");
}

#[allow(dead_code)]
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

fn count_pathways(input: &str) -> usize {
    let mut input_lines = input.lines();
    let first_line = input_lines.next().expect("No lines!");
    let mut paths = vec![0; first_line.len()];
    let beam_idx = first_line.find('S').expect("No 'S' on first line");
    paths[beam_idx] = 1;

    // We can skip 2nd line, and skip every alternate line
    for line in input_lines.skip(1).step_by(2) {
        for (i, c) in line.chars().enumerate() {
            // If an existing beam hits a splitter...
            if c == '^' && paths[i] > 0 {
                // ...add number of paths to either side, and set num paths to 0
                paths[i - 1] += paths[i];
                paths[i + 1] += paths[i];
                paths[i] = 0;
            }
        }
    }
    paths.iter().sum()
}
