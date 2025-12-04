fn main() {
    let input = include_str!("../puzzle_input.txt");

    let num_zeros = count_zeros(input);
    println!("Number of zeros: {}", num_zeros);
}

fn count_zeros(puzzle_input: &str) -> usize {
    let mut counter = 0;
    let mut dial: isize = 50;

    for line in puzzle_input.lines() {
        if line.is_empty() {
            break;
        }

        let (first, rest) = line.split_at(1);
        let amount: isize = rest.parse().expect("Failed to parse into isize");
        let temp_count;
        (dial, temp_count) = match first {
            "R" => rotate_dial(dial, amount),
            "L" => rotate_dial(dial, -amount),
            other => panic!("Unknown character: {other}"),
        };
        counter += temp_count;
    }
    counter
}

#[allow(dead_code)]
fn wrap_dial(mut dial: isize) -> isize {
    if dial > 99 {
        dial % 100
    } else {
        while dial < 0 {
            dial += 100;
        }
        dial
    }
}

fn rotate_dial(dial: isize, amount: isize) -> (isize, usize) {
    let mut counter: usize = 0;
    let full_turns = amount.unsigned_abs() / 100;
    let remainder = amount % 100;

    let mut new_dial = dial + remainder;
    counter += full_turns;

    if new_dial > 99 {
        new_dial -= 100;
        if dial != 0 {
            counter += 1;
        }
    } else if new_dial < 0 {
        new_dial += 100;
        if dial != 0 {
            counter += 1;
        }
    } else if new_dial == 0 {
        counter += 1;
    }

    (new_dial, counter)
}
