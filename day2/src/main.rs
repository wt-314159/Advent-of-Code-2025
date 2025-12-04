fn main() {
    let input = include_str!("../puzzle_input.txt");
    #[allow(unused_variables)]
    let test_input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    let invalid_sum = find_invalid_ids(input);
    println!("Invalid sum = {invalid_sum}");
}

fn find_invalid_ids(input: &str) -> usize {
    let mut total = 0;

    for range in input.split(',') {
        let (start, end) = range
            .split_once('-')
            .expect("Failed to find '-' to split on");
        let start = start.parse::<usize>().expect("Failed to parse start");
        let end = end.trim().parse::<usize>().expect("Failed to parse end");

        // println!("Scanning range: {start}..={end}");
        for num in start..=end {
            // if is_invalid_two(num) {
            //     // println!("Found invalid id: {num}");
            //     total += num;
            // }
            if is_invalid(num) {
                // println!("Found invalid id: {num}");
                total += num;
            }
        }
    }
    total
}

fn is_invalid(num: usize) -> bool {
    let digits = num.to_string();
    let half_len = digits.len() / 2;
    let mut index = 1;
    let mut pattern = &digits[0..index];

    'outer: loop {
        let rem_length = digits.len() - index;
        if rem_length == 0 {
            return false;
        }
        let remainder = rem_length % pattern.len();

        // ID can't be a repeat of pattern if it doesn't
        // divide into it equally
        if remainder != 0 {
            // Add latest digit to pattern and try again
            index += 1;
            // If pattern length is greater than half the
            // length of digits, it can't repeat
            if pattern.len() > half_len {
                return false;
            }
            pattern = &digits[0..index];
            continue;
        }

        let mut ptn_idx = 0;
        for c in digits.chars().skip(index) {
            if c != pattern.chars().nth(ptn_idx).expect("Tried to get nth char") {
                // change pattern to all indexed chars
                pattern = &digits[0..pattern.len() + 1];
                // If pattern length is greater than half the
                // length of digits, it can't repeat
                if pattern.len() > half_len {
                    return false;
                }
                index = pattern.len();
                continue 'outer;
            }

            index += 1; // next character
            ptn_idx += 1;
            // wrap pattern index back to 0
            if ptn_idx >= pattern.len() {
                ptn_idx = 0;
            }
        }
        // Can only reach here if we've checked all characters
        return true;
    }
}

#[allow(dead_code)]
fn is_invalid_two(num: usize) -> bool {
    let digits = num.to_string();
    if !digits.len().is_multiple_of(2) {
        return false;
    }

    let half_idx = digits.len() / 2;
    digits[0..half_idx] == digits[half_idx..]
}
