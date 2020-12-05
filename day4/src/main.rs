use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{self, BufRead, Read};
use std::iter::once;
use std::mem::swap;

fn validate_passport_part1(input: &String) -> Result<bool, Box<dyn Error>> {
    let pieces = input
        .split(|c| c == '\n' || c == ' ')
        .map(|s| s.split(':').next().ok_or("missing a field identifier"))
        .collect::<Result<HashSet<_>, _>>()?;
    for field in &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"] {
        if !pieces.contains(field) {
            return Ok(false);
        }
    }

    Ok(true)
}

fn validate_passport_part2(input: &String) -> Result<bool, Box<dyn Error>> {
    let pieces = input
        .split(|c| c == '\n' || c == ' ')
        .filter(|s| *s != "")
        .map(|s| {
            let mut parts = s.split(':');
            let field_part = parts.next();
            let val_part = parts.next();
            if let (Some(field), Some(val)) = (field_part, val_part) {
                Ok((field, val))
            } else {
                Err("Field identifier or value not found")
            }
        })
        .collect::<Result<HashMap<_, _>, _>>()?;

    if !pieces
        .get("byr")
        .and_then(|v| v.parse::<u64>().ok())
        .map(|v| v >= 1920 && v <= 2002)
        .unwrap_or(false)
    {
        return Ok(false);
    }

    if !pieces
        .get("iyr")
        .and_then(|v| v.parse::<u64>().ok())
        .map(|v| v >= 2010 && v <= 2020)
        .unwrap_or(false)
    {
        return Ok(false);
    }

    if !pieces
        .get("eyr")
        .and_then(|v| v.parse::<u64>().ok())
        .map(|v| v >= 2020 && v <= 2030)
        .unwrap_or(false)
    {
        return Ok(false);
    }

    if !pieces
        .get("ecl")
        .map(|v| ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(v))
        .unwrap_or(false)
    {
        return Ok(false);
    }

    let hgt_piece = if let Some(p) = pieces.get("hgt") {
        p.chars().collect::<Vec<_>>()
    } else {
        return Ok(false);
    };
    let num_len = hgt_piece.len() - 2;
    let hgt_num_piece = if let Ok(v) = hgt_piece
        .iter()
        .take(num_len)
        .collect::<String>()
        .parse::<u64>()
    {
        v
    } else {
        return Ok(false);
    };

    if &hgt_piece[num_len..] == ['i', 'n'] {
        if hgt_num_piece < 59 || hgt_num_piece > 76 {
            return Ok(false);
        }
    } else if &hgt_piece[num_len..] == ['c', 'm'] {
        if hgt_num_piece < 150 || hgt_num_piece > 193 {
            return Ok(false);
        }
    } else {
        return Ok(false);
    }

    let mut hcl_piece = if let Some(p) = pieces.get("hcl") {
        p.chars()
    } else {
        return Ok(false);
    };
    if let Some('#') = hcl_piece.next() {
    } else {
        return Ok(false);
    }

    let hcl = hcl_piece.collect::<Vec<char>>();
    if hcl.len() != 6
        || !hcl
            .iter()
            .all(|c| c.is_ascii_digit() || c.is_ascii_lowercase())
    {
        return Ok(false);
    }

    let pid = if let Some(p) = pieces.get("pid") {
        p.chars().collect::<Vec<char>>()
    } else {
        return Ok(false);
    };
    if !pid.iter().all(|c| c.is_numeric()) {
        return Ok(false);
    }

    if pid.len() != 9 {
        return Ok(false);
    }

    Ok(true)
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = io::stdin();
    let stdin = stdin.lock();

    let mut lines = stdin.lines();

    let mut passports = lines
        .chain(once(Ok(String::new())))
        .scan(String::new(), |state, r| {
            Some(r.map(|s| {
                if s.is_empty() {
                    let mut out = String::new();
                    swap(state, &mut out);
                    Some(out)
                } else {
                    state.push_str(" ");
                    state.push_str(&s);
                    None
                }
            }))
        })
        .filter_map(|r| match r {
            Ok(Some(s)) => Some(Ok(s)),
            Ok(None) => None,
            Err(e) => Some(Err(e)),
        })
        .collect::<Result<Vec<String>, _>>()?;

    let mut valid_passports_part1 = 0;

    for p in passports.iter() {
        if validate_passport_part1(p)? {
            valid_passports_part1 += 1;
        }
    }

    let mut valid_passports_part2 = 0;
    for p in passports.iter() {
        if validate_passport_part2(p)? {
            valid_passports_part2 += 1;
        }
    }

    println!("1) Number of valid passports: {}", valid_passports_part1);
    println!("2) Number of valid passports: {}", valid_passports_part2);
    Ok(())
}
