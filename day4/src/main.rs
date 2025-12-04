use std::{cmp, iter};

fn main() {
    let input = include_str!("../puzzle_input.txt");

    #[allow(unused_variables)]
    let test_input = r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;

    let mut grid = create_grid(input);
    // print_grid(&grid);
    println!();
    // let num_movable = count_accessible_rolls(&grid);
    // println!("Number of rolls that can be moved: {num_movable}");
    let total_moved = remove_rolls(&mut grid);
    println!("Total moved: {total_moved}");
}

#[allow(dead_code)]
fn count_accessible_rolls(grid: &[Vec<bool>]) -> usize {
    let mut count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] && get_surrounding_cells(grid, i, j).filter(|b| **b).count() < 4 {
                count += 1;
                // print!("x");
            }
            // else {
            // let c = match grid[i][j] {
            //     true => '@',
            //     false => '.',
            // };
            // print!("{c}");
            // }
        }
        // println!();
    }
    count
}

fn remove_rolls(grid: &mut [Vec<bool>]) -> usize {
    let mut count = 0;
    let mut to_remove = Vec::with_capacity(grid.len() * grid[0].len());
    loop {
        to_remove.clear();
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] && get_surrounding_cells(grid, i, j).filter(|b| **b).count() < 4 {
                    to_remove.push((i, j));
                }
            }
        }

        // println!("After removing {} rolls:", to_remove.len());
        if to_remove.is_empty() {
            return count;
        } else {
            count += to_remove.len();
        }
        while let Some((i, j)) = to_remove.pop() {
            grid[i][j] = false;
        }
        // print_grid(grid);
    }
}

fn create_grid(input: &str) -> Vec<Vec<bool>> {
    let mut grid = Vec::new();
    for line in input.lines() {
        let row = line
            .chars()
            .map(|c| match c {
                '@' => true,
                '.' => false,
                wat => panic!("Failed to parse character '{wat}'"),
            })
            .collect();
        grid.push(row);
    }
    grid
}

fn get_surrounding_cells(
    grid: &[Vec<bool>],
    row: usize,
    col: usize,
) -> impl Iterator<Item = &bool> {
    get_surrounding_indices(grid, row, col).map(|(_, (_, b))| b)
}

fn get_surrounding_indices(
    grid: &[Vec<bool>],
    row: usize,
    col: usize,
) -> impl Iterator<Item = (usize, (usize, &bool))> {
    let (min_row, max_row) = (
        cmp::max(0, (row as isize) - 1) as usize,
        cmp::min(grid.len(), row + 2),
    );
    let (min_col, max_col) = (
        cmp::max(0, (col as isize) - 1) as usize,
        cmp::min(grid[0].len(), col + 2),
    );
    let cur_row = row - min_row;
    let cur_col = col - min_col;

    grid[min_row..max_row]
        .iter()
        .enumerate()
        .flat_map(move |(i, r)| iter::repeat(i).zip(r[min_col..max_col].iter().enumerate()))
        .filter(move |(i, (j, _))| *i != cur_row || *j != cur_col)
}

#[allow(dead_code)]
fn print_grid(grid: &[Vec<bool>]) {
    for row in grid {
        for b in row {
            let c = match b {
                true => '@',
                false => '.',
            };
            print!("{c}");
        }
        println!();
    }
}
