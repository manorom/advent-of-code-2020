use std::error::Error;
use std::io::{stdin, BufRead};

#[derive(Clone, Copy, Debug, PartialEq)]
enum Position {
    Empty,
    Filled,
    Floor,
}

impl From<char> for Position {
    fn from(c: char) -> Self {
        match c {
            '.' => Position::Floor,
            'L' => Position::Empty,
            '#' => Position::Floor,
            _ => panic!("Malformed input, expected . L or #"),
        }
    }
}

fn filled_adjacent_seats(grid: &Vec<Vec<Position>>, i: usize, j: usize) -> u32 {
    let mut adjacent_filled_seats = 0;
    if i.checked_sub(1).is_some() {
        if grid[i - 1][j] == Position::Filled {
            adjacent_filled_seats += 1;
        }
        if j.checked_sub(1).is_some() && grid[i - 1][j - 1] == Position::Filled {
            adjacent_filled_seats += 1;
        }

        if (j + 1) < grid[i].len() && grid[i - 1][j + 1] == Position::Filled {
            adjacent_filled_seats += 1;
        }
    }

    if (i + 1) < grid.len() {
        if grid[i + 1][j] == Position::Filled {
            adjacent_filled_seats += 1;
        }
        if j.checked_sub(1).is_some() && grid[i + 1][j - 1] == Position::Filled {
            adjacent_filled_seats += 1;
        }
        if (j + 1) < grid[i].len() && grid[i + 1][j + 1] == Position::Filled {
            adjacent_filled_seats += 1;
        }
    }

    if j.checked_sub(1).is_some() && grid[i][j - 1] == Position::Filled {
        adjacent_filled_seats += 1;
    }

    if (j + 1) < grid[i].len() && grid[i][j + 1] == Position::Filled {
        adjacent_filled_seats += 1;
    }

    return adjacent_filled_seats;
}

fn filled_visible_seats(grid: &Vec<Vec<Position>>, i: usize, j: usize) -> u32 {
    let i = i as i64;
    let j = j as i64;
    let mut visible_filled_seats = 0;
    for (dir_i, dir_j) in &[
        (1, 0),
        (0, 1),
        (1, 1),
        (-1, 0),
        (0, -1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ] {
        let mut visit_i = i + dir_i;
        let mut visit_j = j + dir_j;
        while visit_i >= 0
            && visit_i < (grid.len() as i64)
            && visit_j >= 0
            && visit_j < (grid[i as usize].len() as i64)
        {
            if grid[visit_i as usize][visit_j as usize] == Position::Filled {
                visible_filled_seats += 1;
                break;
            }
            if grid[visit_i as usize][visit_j as usize] == Position::Empty {
                break;
            }
            visit_i += dir_i;
            visit_j += dir_j;
        }
    }

    return visible_filled_seats;
}

fn main() {
    let input = include_str!("input.txt");
    let mut grid = input
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().map(|c| Position::from(c)).collect::<Vec<_>>())
        .collect::<Vec<Vec<_>>>();

    let mut stabilised = false;
    let mut num_iters = -1;
    while !stabilised {
        stabilised = true;
        num_iters += 1;
        let mut new_grid = grid.clone();

        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == Position::Empty {
                    if filled_visible_seats(&grid, i, j) == 0 {
                        new_grid[i][j] = Position::Filled;
                        stabilised = false;
                    }
                } else if grid[i][j] == Position::Filled {
                    if filled_visible_seats(&grid, i, j) >= 5 {
                        new_grid[i][j] = Position::Empty;
                        stabilised = false;
                    }
                }
            }
        }

        grid = new_grid.clone();
    }

    let num_filled_seats = grid
        .iter()
        .map(|v| v.iter())
        .flatten()
        .filter(|&&c| c == Position::Filled)
        .count();
    println!(
        "stabilised after {} iterations with {} filled seats",
        num_iters, num_filled_seats
    );
}
