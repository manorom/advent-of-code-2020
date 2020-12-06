use std::io::{BufRead, BufReader, Lines, Seek, SeekFrom, Read};
use std::iter::successors;
use std::fs::File;
use std::error::Error;
use std::cmp::{min, max};
use std::env::args;

fn find_free_seat(lines: Lines<impl BufRead>, min_seat: u64, max_seat: u64) -> Result<String, Box<dyn Error>> {
    // because there are 8 columns, there is an even number of sets
    // assuming that the plane is fully booked!
    let mut seat_index = [(0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0), (0,0)]; 

    for line in lines {
        let line = line?;
        for (idx, c) in line.chars().enumerate() {
            if c == 'F' || c == 'L' {
                seat_index[idx].0 += 1;
            } else {
                seat_index[idx].1 += 1;
            }
        }
    }


    for mut seat in (0..min_seat) {
        for (idx, val) in successors(Some(512), |i| Some(i / 2)).take(10).enumerate() {
            if seat >= val {
                seat_index[idx].1 += 1;
                seat -= val
            } else {
                seat_index[idx].0 += 1;
            }
        }
    }

    for mut seat in (max_seat+1..1024) {
        for (idx, val) in successors(Some(512), |i| Some(i / 2)).take(10).enumerate() {
            if seat >= val {
                seat_index[idx].1 += 1;
                seat -= val
            } else {
                seat_index[idx].0 += 1;
            }
        }
        
    }
    
    let row_coords = seat_index[..7].iter().map(|(f, b)| {
        if f < b {
            'F'
        } else {
            'B'
        }
    }).collect::<String>();

    let column_coords = seat_index[7..].iter().map(|(l, r)| {
        if l < r {
            'L'
        } else {
            'R'
        }
    }).collect::<String>();
    Ok(row_coords + &column_coords)
}

fn gen_seat_id(coords: &str) -> u64 {
    let row = coords.chars().take(7).fold((0,128), |(f,b), c| {
        if c == 'F' {
            (f, b - (b - f)/2)
        } else {
            (f + (b - f)/2, b)
        }
    }).0;

    let column = coords.chars().skip(7).fold((0, 8), |(l, r), c| {
        if c == 'L' {
            (l, r - (r - l)/2)
        } else {
            (l + (r -l)/2, r)
        }
    }).0;
    
    row * 8 + column
}

fn min_max_seat_id(lines: Lines<impl BufRead>) -> Result<(u64, u64), Box<dyn Error>> {

    Ok(lines.map(|coords| coords.map(|c| gen_seat_id(&c))).try_fold((1024,0), |(min_id, max_id), v| {
        if let Ok(id) = v {
            Some((min(min_id, id), max(max_id, id)))
        } else {
            None
        }
    }).ok_or("No seat coordiantes found")?)
}

fn main() -> Result<(), Box<dyn Error>> {
    
    let input_file_name = args().nth(1).ok_or("Specify the input file name as first argument")?;
    let input_file = File::open(input_file_name)?;
    let mut input_buffer = BufReader::new(input_file);
    
    let (min_seat_id, max_seat_id) = min_max_seat_id(input_buffer.by_ref().lines())?;
    println!("Lowest seat id is: {}", min_seat_id);
    println!("Hightest seat id is: {}", max_seat_id);
    
    input_buffer.seek(SeekFrom::Start(0));
    dbg!(gen_seat_id(&find_free_seat(input_buffer.by_ref().lines(), min_seat_id, max_seat_id)?));
    Ok(())
}


#[cfg(test)]
mod tests {
    #[test]
    fn example_seat_id_gen() {
        assert_eq!(super::gen_seat_id("BFFFBBFRRR"), 567);
        assert_eq!(super::gen_seat_id("FFFBBBFRRR"), 119);
        assert_eq!(super::gen_seat_id("BBFFBBFRLL"), 820);
    }
}
