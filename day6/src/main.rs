use std::io::stdin;
use std::error::Error;
use std::io::BufRead;
use std::iter::Iterator;


fn count_answers<'a>(iter: impl Iterator<Item=&'a String>, quorum: usize) -> u64 {
    let mut index = [0; 26];

    for s in iter {
        for c in s.chars() {
            let i = c as u64 - 97;
            index[i as usize] += 1;
        }
    }

    index.iter()
        .map(|v| if *v >= quorum {
            1
        } else {
            0
        })
        .sum()
}


fn main() -> Result<(), Box<dyn Error>> {
    let stdin = stdin();
    let stdin = stdin.lock();

    let mut lines = stdin.lines();
    let mut entries = Vec::new();
    
    while let Some(s) = Some(lines.by_ref().take_while(|l| {
        if let Ok(s) = l {
            !s.is_empty()
        } else {
            false
        }
    }).collect::<Result<Vec<_>, _>>()?).and_then(|l| {
        if l.is_empty() {
            None
        } else {
            Some(l)
        }
    }) {

        entries.push(s);
    }

    
    let num_questions_p1: u64 = entries.iter().map(|v| count_answers(v.iter(), 1)).sum();
    println!("Number of questions for part1: {}", num_questions_p1);

    let num_questions_p2: u64 = entries.iter().map(|v| {
        let quorum = v.len();
        count_answers(v.iter(), quorum)
        }).sum();
    println!("Number of questions for part2: {}", num_questions_p2);
    Ok(())
}

