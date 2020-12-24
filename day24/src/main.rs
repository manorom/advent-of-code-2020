use std::collections::HashSet;
use std::iter::{from_fn, Iterator};

#[derive(Debug)]
enum Direction {
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
    NorthEast,
}

impl Direction {
    fn calc_move(&self) -> (i64, i64) {
        match self {
            Direction::East => (2, 0),
            Direction::West => (-2, 0),
            Direction::SouthWest => (-1, 1),
            Direction::SouthEast => (1, 1),
            Direction::NorthWest => (-1, -1),
            Direction::NorthEast => (1, -1),
        }
    }
    fn parse(input: &str) -> Vec<Direction> {
        let mut input_iter = input.chars().peekable();
        let mut parsed = Vec::new();
        while let Some(d) = input_iter.peek() {
            match d {
                'e' => {
                    input_iter.next();
                    parsed.push(Direction::East);
                }
                's' => {
                    input_iter.next();
                    match input_iter.next() {
                        Some('e') => parsed.push(Direction::SouthEast),
                        Some('w') => parsed.push(Direction::SouthWest),
                        Some(e) => {
                            panic!("encountered unknown direction s{}", e);
                        }
                        None => {
                            panic!("Encountered lonely s direction");
                        }
                    }
                }
                'w' => {
                    input_iter.next();
                    parsed.push(Direction::West);
                }
                'n' => {
                    input_iter.next();
                    match input_iter.next() {
                        Some('e') => parsed.push(Direction::NorthEast),
                        Some('w') => parsed.push(Direction::NorthWest),
                        Some(e) => {
                            panic!("encountered unknown direction n{}", e);
                        }
                        None => {
                            panic!("Encountered lonely n direction");
                        }
                    }
                }
                c => {
                    panic!("Unknown direction {}", c);
                }
            }
        }
        parsed
    }
}

const POSITON_OFFSETS: [(i64, i64); 6] = [(2, 0), (-2, 0), (-1, -1), (-1, 1), (1, -1), (1, 1)];

fn point_neighbors(point: (i64, i64)) -> impl Iterator<Item = (i64, i64)> {
    let mut offsets_iter = POSITON_OFFSETS.iter();
    from_fn(move || {
        offsets_iter
            .next()
            .map(|offset| (point.0 + offset.0, point.1 + offset.1))
    })
}

fn get_floor_dimensions(floor: &HashSet<(i64, i64)>) -> ((i64, i64), (i64, i64)) {
    let topmost = floor.iter().map(|(i, _)| i).min().unwrap();
    let leftmost = floor.iter().map(|(_, j)| j).min().unwrap();
    let bottommost = floor.iter().map(|(i, _)| i).max().unwrap();
    let rightmost = floor.iter().map(|(_, j)| j).max().unwrap();

    ((topmost - 2, leftmost - 2), (bottommost + 2, rightmost + 2))
}

fn main() {
    let directions = include_str!("input.txt")
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| Direction::parse(s))
        .collect::<Vec<_>>();
    let mut black_tiles_map: HashSet<(i64, i64)> = HashSet::new();
    for line in directions {
        let pos = line.iter().fold((0, 0), |accum, d| {
            let tile_move = d.calc_move();
            (accum.0 + tile_move.0, accum.1 + tile_move.1)
        });

        if black_tiles_map.contains(&pos) {
            black_tiles_map.remove(&pos);
        } else {
            black_tiles_map.insert(pos);
        }
    }
    println!(
        "Number of black tiles following instructions (part 1): {}",
        black_tiles_map.len()
    );

    for _ in 0..100 {
        let black_tiles_map_front = black_tiles_map.clone();

        let ((ifrom, jfrom), (ito, jto)) = get_floor_dimensions(&black_tiles_map_front);
        for i in ifrom..=ito {
            for j in jfrom..=jto {
                let num_adj_black_tiles = point_neighbors((i, j))
                    .map(|(k, l)| black_tiles_map_front.contains(&(k, l)))
                    .filter(|b| *b)
                    .count();
                if black_tiles_map_front.contains(&(i, j)) {
                    if num_adj_black_tiles == 0 || num_adj_black_tiles > 2 {
                        black_tiles_map.remove(&(i, j));
                    }
                } else {
                    if num_adj_black_tiles == 2 {
                        black_tiles_map.insert((i, j));
                    }
                }
            }
        }
    }
    println!(
        "Number of black tiles after 100 rounds of flipping (part 2): {}",
        black_tiles_map.len()
    );
}
