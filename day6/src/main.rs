use std::error::Error;
use std::io::stdin;
use std::io::BufRead;
use std::iter::Iterator;

fn count_answers<'a>(iter: impl Iterator<Item = &'a String>, quorum: usize) -> usize {
    let mut index = [0; 26];

    for c in iter.map(|s| s.chars()).flatten() {
        index[c as usize - 97 as usize] +=1;
    }
    index.iter().filter(|&&v| v >= quorum).count()
}

fn main() -> Result<(), Box<dyn Error>> {
    let stdin = stdin();
    let stdin = stdin.lock();

    let mut lines = stdin.lines();
    let mut entries: Vec<Vec<String>> = Vec::new();

    while let Some(s) = Some(
        lines
            .by_ref()
            .take_while(|l| if let Ok(s) = l { !s.is_empty() } else { false })
            .collect::<Result<Vec<_>, _>>()?,
    )
    .and_then(|l| if l.is_empty() { None } else { Some(l) })
    {
        entries.push(s);
    }

    let num_questions_p1: usize = entries.iter().map(|v| count_answers(v.iter(), 1)).sum();
    println!("Number of questions for part1: {}", num_questions_p1);

    let num_questions_p2: usize = entries
        .iter()
        .map(|v| {
            let quorum = v.len();
            count_answers(v.iter(), quorum)
        })
        .sum();
    println!("Number of questions for part2: {}", num_questions_p2);
    Ok(())
}
