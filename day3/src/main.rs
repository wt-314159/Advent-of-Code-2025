fn main() {
    let input = include_str!("../puzzle_input.txt");
    #[allow(unused_variables)]
    let test_input = r"987654321111111
811111111111119
234234234234278
818181911112111";

    let total_joltage = find_total_joltage(input);
    println!("Total joltage = {total_joltage}")
}

fn find_total_joltage(input: &str) -> usize {
    let mut total = 0;
    for bank in input.lines() {
        let max = find_max_joltage(bank);
        total += max;
    }
    total
}

fn find_max_joltage(bank: &str) -> usize {
    // find biggest digit in all but last character
    let slice = &bank[..bank.len() - 1];
    let max = find_max_digit(slice);
    // Find biggest digit in rest of string
    let second_max = find_max_digit(&bank[max.0 + 1..]);
    let tens = char_to_num(max.1);
    let ones = char_to_num(second_max.1);
    tens * 10 + ones
}

fn char_to_num(c: char) -> usize {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        wat => panic!("Unexpected digit that can't be parsed: '{wat}'"),
    }
}

fn find_max_digit(bank: &str) -> (usize, char) {
    let mut max = '0';
    let mut max_idx = 0;
    for (i, c) in bank.chars().enumerate() {
        if c > max {
            max = c;
            max_idx = i;
        }
    }
    (max_idx, max)
}
