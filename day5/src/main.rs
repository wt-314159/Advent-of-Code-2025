use std::collections::HashSet;

fn main() {
    let input = include_str!("../puzzle_input.txt");

    #[allow(unused_variables)]
    let test_input = r"3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    let result = count_all_fresh(input);
    println!("Result {result}");
}

#[allow(dead_code)]
fn count_fresh_ingredients(input: &str) -> usize {
    let (ranges, ingredients) = input.split_once("\n\n").expect("Failed to find blank line");
    let ranges = find_fresh_ranges(ranges);

    let mut count = 0;
    for line in ingredients.lines() {
        let id: usize = line.parse().expect("Failed to parse ID");
        for range in &ranges {
            if id >= range.start && id <= range.end {
                count += 1;
                break;
            }
        }
    }
    count
}

fn count_all_fresh(input: &str) -> usize {
    let (ranges, _) = input.split_once("\n\n").expect("Failed to find blank line");

    let mut ranges = find_fresh_ranges(ranges);
    count_all_in_ranges(&mut ranges)
}

#[allow(dead_code)]
fn count_all_in_ranges_naive(ranges: &[Range]) -> usize {
    let mut fresh_set = HashSet::new();
    for range in ranges {
        for i in range.start..=range.end {
            fresh_set.insert(i);
        }
    }
    fresh_set.len()
}

fn count_all_in_ranges(ranges: &mut [Range]) -> usize {
    let mut total = 0;
    'outer: for i in 0..ranges.len() {
        let mut range = ranges[i];
        // adjust for overlap with previous ranges
        for prev_range in &ranges[0..i] {
            // whole range is overlapped
            if range.start >= prev_range.start && range.end <= prev_range.end {
                continue 'outer; // skip as no new IDs
            }
            // start is within other range, adjust start to other ranges end + 1
            if range.start >= prev_range.start && range.start <= prev_range.end {
                range.start = prev_range.end + 1;
            }
            // end is within other range, adjust end to other range's start - 1
            else if range.end >= prev_range.start && range.end <= prev_range.end {
                range.end = prev_range.start - 1;
            }
            // check if current range encapsulates previous range
            if prev_range.start >= range.start && prev_range.end <= range.end && prev_range.counted
            {
                total -= prev_range.count();
            }
        }
        // add count of current range to total
        total += range.count();
        range.counted = true;
        // Update range in ranges slice
        ranges[i] = range;
    }
    total
}

fn find_fresh_ranges(input: &str) -> Vec<Range> {
    let mut ranges = Vec::new();
    for line in input.lines() {
        let (start, end) = line
            .split_once('-')
            .expect("Failed to find range separator.");
        let (start, end) = (start.parse(), end.parse());
        let (start, end) = (
            start.expect("Failed to parse start"),
            end.expect("Failed to parse end"),
        );
        ranges.push(Range::new(start, end));
    }
    ranges
}

#[allow(dead_code)]
fn print_ranges(ranges: &[Range]) {
    for range in ranges {
        println!("{}-{}", range.start, range.end);
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Range {
    start: usize,
    end: usize,
    counted: bool,
}

impl Range {
    fn new(start: usize, end: usize) -> Range {
        Range {
            start,
            end,
            counted: false,
        }
    }

    fn count(&self) -> usize {
        if self.start > self.end {
            return 0;
        }
        self.end - self.start + 1
    }
}
