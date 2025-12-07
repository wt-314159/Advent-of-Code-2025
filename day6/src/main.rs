fn main() {
    let input = include_str!("../puzzle_input.txt");

    #[allow(unused_variables)]
    let test_input = r"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    // let (problem_nums, operations) = get_problems(input);
    // let total = count_total(problem_nums, operations);
    let total = solve_problems_two(input);
    println!("Total = {total}");
}

#[allow(dead_code)]
fn count_total(problem_nums: Vec<Vec<usize>>, operations: Vec<Operation>) -> usize {
    let mut total = 0;
    // We can assume all the lines have the same number of numbers and operations
    for i in 0..problem_nums[0].len() {
        let op = operations[i];
        let mut cur_total = op.start_value();

        for line in &problem_nums {
            cur_total = op.apply(cur_total, line[i]);
        }
        total += cur_total;
    }
    total
}

#[allow(dead_code)]
fn get_problems(input: &str) -> (Vec<Vec<usize>>, Vec<Operation>) {
    let mut problem_nums: Vec<Vec<usize>> = Vec::new();
    let mut line_iter = input.lines().peekable();
    while let Some(line) = line_iter.peek()
        && line.trim_start().starts_with(|c: char| c.is_ascii_digit())
    {
        // We've already peeked, so this if fine
        let line = line_iter.next().unwrap();
        let nums = line
            .split_whitespace()
            .map(|num| num.parse::<usize>().expect("Failed to parse num"))
            .collect();
        problem_nums.push(nums);
    }

    let operations: Vec<Operation> = line_iter
        .next()
        .expect("No line of operations found!")
        .split_whitespace()
        .map(Operation::parse)
        .collect();
    (problem_nums, operations)
}

fn solve_problems_two(input: &str) -> usize {
    let last_line = input.lines().last().expect("Couldn't find last line.");
    // Need to iterate through the line, and find the operations
    // as well as spacing after
    let mut ranges = Vec::new();
    let mut ops = Vec::new();
    let mut idx = 0;
    while let Some(mut next_op) = last_line[idx + 1..].find(|c: char| !c.is_ascii_whitespace()) {
        next_op += idx;
        ranges.push(idx..next_op);
        ops.push(Operation::parse(&last_line[idx..idx + 1]));
        idx = next_op + 1;
    }
    ranges.push(idx..last_line.len());
    ops.push(Operation::parse(&last_line[idx..idx + 1]));

    // Get a vector of each vertical column of numbers, including extra spaces
    let mut num_slices = vec![Vec::new(); ranges.len()];
    let mut line_iter = input.lines().peekable();
    while let Some(line) = line_iter.peek()
        && line.trim_start().starts_with(|c: char| c.is_ascii_digit())
    {
        let line = line_iter.next().unwrap();
        for i in 0..ranges.len() {
            let range = &ranges[i];
            num_slices[i].push(&line[range.clone()]);
        }
    }

    let mut total = 0;
    for i in 0..ranges.len() {
        let op = ops[i];
        let mut cur_total = op.start_value();
        // print!("{cur_total}");
        for num in get_numbers_from_slices(&num_slices[i]) {
            cur_total = op.apply(cur_total, num);
            // print!(" {} {}", op.char(), num)
        }
        // println!(" = {cur_total}");
        total += cur_total;
    }
    total
}

fn get_numbers_from_slices(num_slices: &[&str]) -> Vec<usize> {
    let mut numbers = Vec::with_capacity(num_slices.len());
    for idx in 0..num_slices[0].len() {
        let mut cur_num = 0;
        for slice in num_slices {
            if let Some(c) = slice.chars().nth(idx)
                && let Some(digit) = c.to_digit(10)
            {
                cur_num = cur_num * 10 + digit;
            }
        }
        numbers.push(cur_num as usize);
    }
    numbers
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Operation {
    Add,
    Mul,
}

impl Operation {
    fn parse(op: &str) -> Operation {
        match op {
            "+" => Self::Add,
            "*" => Self::Mul,
            huh => panic!("Couldn't parse {huh} to Operation"),
        }
    }

    fn start_value(&self) -> usize {
        match self {
            Self::Add => 0,
            Self::Mul => 1,
        }
    }

    fn apply(&self, val1: usize, val2: usize) -> usize {
        match self {
            Self::Add => val1 + val2,
            Self::Mul => val1 * val2,
        }
    }

    fn char(&self) -> char {
        match self {
            Self::Add => '+',
            Self::Mul => '*',
        }
    }
}
