use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Rule {
    Terminal(char),
    NonTerminal(Vec<Vec<u64>>),
}

impl Rule {
    fn parse_product(prod: &str) -> Vec<u64> {
        prod.split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.trim())
            .map(|s| s.parse::<u64>().unwrap())
            .collect::<Vec<u64>>()
    }
    fn parse(input: &str) -> (u64, Rule) {
        let mut pieces = input.split(':').map(|s| s.trim());
        let rulenum = pieces.next().unwrap().parse::<u64>().unwrap();
        let rule = pieces.next().unwrap().trim();
        if rule.chars().next().unwrap() == '\"' {
            let c = rule.chars().nth(1).unwrap();
            (rulenum, Rule::Terminal(c))
        } else {
            let variants = rule
                .split('|')
                .map(|s| s.trim())
                .map(|s| Rule::parse_product(s))
                .collect::<Vec<_>>();
            (rulenum, Rule::NonTerminal(variants))
        }
    }
}

fn pda_match(grammar: &HashMap<u64, Rule>, word: &[char], mut pdstack: Vec<u64>) -> bool {
    if word.len() == 0 && pdstack.len() == 0 {
        return true;
    } else if word.len() == 0 || pdstack.len() == 0 {
        return false;
    }

    let use_rule = pdstack.pop().unwrap();
    return match &grammar[&use_rule] {
        Rule::Terminal(c) => {
            if *c == word[0] {
                pda_match(grammar, &word[1..], pdstack)
            } else {
                false
            }
        }
        Rule::NonTerminal(variants) => {
            for variant in variants.iter() {
                let new_stack = pdstack
                    .iter()
                    .cloned()
                    .chain(variant.iter().rev().cloned())
                    .collect::<Vec<_>>();
                let is_match = pda_match(grammar, word, new_stack);
                if is_match {
                    return true;
                }
            }
            false
        }
    };
}

fn parse_input(input: &str) -> (HashMap<u64, Rule>, Vec<Vec<char>>) {
    let mut input_iter = input.split('\n');
    let rules = input_iter
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|l| Rule::parse(l))
        .collect::<HashMap<u64, Rule>>();
    let messages = input_iter
        .filter(|s| !s.is_empty())
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    (rules, messages)
}

fn main() {
    let (mut grammar, messages) = parse_input(include_str!("input.txt"));
    println!(
        "Number of message that match without loop rule: {}",
        messages
            .iter()
            .map(|s| pda_match(&grammar, &s, vec![0]))
            .filter(|b| *b)
            .count()
    );
    grammar.insert(8, Rule::NonTerminal(vec![vec![42], vec![42, 8]]));
    grammar.insert(11, Rule::NonTerminal(vec![vec![42, 31], vec![42, 11, 31]]));
    println!(
        "Number of message that match with loop rule: {}",
        messages
            .iter()
            .map(|s| pda_match(&grammar, &s, vec![0]))
            .filter(|b| *b)
            .count()
    );
}
