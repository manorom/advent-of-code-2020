use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::{Iterator, from_fn};

fn repeated_line_pattern(line: &str) -> impl Iterator<Item=char> + '_ {
    let mut line_iter = line.chars();
    from_fn(move || {
        if let Some(c) = line_iter.next() {
            Some(c)
        } else {
            line_iter = line.chars();
            line_iter.next()
        }
    })
}

fn trees_hit(right: usize, down: usize, patterns: impl Iterator<Item=impl Iterator<Item=char>>) -> Result<u64, Box<dyn Error>> {
    Ok(patterns
        .step_by(down)
        .enumerate()
        .map(|(i, mut pattern)| {
            let c = pattern.nth(i * right)?;
            if c == '#' {
                Some(1)
            } else if c == '.' {
                Some(0)
            } else {
                None
            }
        })
        .try_fold(0, |accum, x| {
            x.map(|v| accum + v).ok_or("returned None")
        })?) 
}

fn main() -> Result<(), Box<dyn Error>> {
   let file = File::open("input.txt")?;
   let file_lines = BufReader::new(file)
       .lines()
       .collect::<Result<Vec<_>,_>>()?;

    let line_patterns = file_lines.iter().map(|s| repeated_line_pattern(s));
    println!("Number of trees hit with slope (3, 1): {}", trees_hit(3, 1, line_patterns.clone())?);

    let prod: u64 = [(1,1), (3,1), (5,1), (7,1), (1, 2)]
        .iter()
        .map(|(right,down)| trees_hit(*right, *down, line_patterns.clone()))
        .try_fold(1, |accum, x| {
            x.map(|v| accum * v)
        })?;
    println!("Product of all tree collisions is: {}", prod);

    Ok(())
}
