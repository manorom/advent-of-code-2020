use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct Rule {
    name: String,
    range1: (u64, u64),
    range2: (u64, u64),
}

impl Rule {
    fn parse_range(line: &str) -> (u64, u64) {
        let mut pieces = line.split("-");
        (
            pieces.next().unwrap().parse::<u64>().unwrap(),
            pieces.next().unwrap().parse::<u64>().unwrap(),
        )
    }
    fn parse(line: &str) -> Rule {
        let mut pieces = line.split(":");
        let name = pieces.next().unwrap().to_string();
        let mut pieces = pieces.next().unwrap().split(" ").filter(|l| !l.is_empty());
        let range1 = Rule::parse_range(pieces.next().unwrap());
        pieces.next();
        let range2 = Rule::parse_range(pieces.next().unwrap());
        Rule {
            name,
            range1,
            range2,
        }
    }

    fn check(&self, num: u64) -> bool {
        if num >= self.range1.0 && num <= self.range1.1 {
            true
        } else if num >= self.range2.0 && num <= self.range2.1 {
            true
        } else {
            false
        }
    }
}

trait RuleSet {
    fn check_any(&self, num: u64) -> bool;
}

impl RuleSet for Vec<Rule> {
    fn check_any(&self, num: u64) -> bool {
        self.iter().map(|rule| rule.check(num)).any(|c| c)
    }
}

fn parse_input(input: &str) -> (Vec<Rule>, Vec<u64>, Vec<Vec<u64>>) {
    let mut input = input.split("\n");
    let rules = input
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|l| Rule::parse(l))
        .collect::<Vec<_>>();
    input.next();
    let my_ticket = input
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    input.next();
    input.next();
    let nearby_tickets = input
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.split(",")
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    (rules, my_ticket, nearby_tickets)
}

fn main() {
    let input = include_str!("input.txt");

    let (rules, my_ticket, nearby_tickets) = parse_input(input);
    let ticket_error_rate = nearby_tickets.iter().fold(0, |mut ter, ticket| {
        for entry in ticket.iter() {
            if !rules.check_any(*entry) {
                ter += *entry;
            }
        }
        ter
    });
    println!("Ticket error rate is: {}", ticket_error_rate);

    let valid_nearby_tickets = nearby_tickets
        .iter()
        .filter(|ticket| ticket.iter().all(|entry| rules.check_any(*entry)))
        .collect::<Vec<_>>();

    let mut rules_order = vec![HashSet::new(); valid_nearby_tickets[0].len()];

    for i in 0..valid_nearby_tickets[0].len() {
        for (rule_idx, rule) in rules.iter().enumerate() {
            if valid_nearby_tickets
                .iter()
                .map(|ticket| rule.check(ticket[i]))
                .all(|p| p)
            {
                rules_order[i].insert(rule_idx);
            }
        }
    }

    let mut sorted_rules_order = rules_order.iter_mut().enumerate().collect::<Vec<_>>();
    sorted_rules_order.sort_by_key(|(_, s)| s.len());
    let mut remove_rules = Vec::new();
    for (idx, set) in sorted_rules_order.iter_mut() {
        for remove_rule in remove_rules.iter() {
            set.remove(remove_rule);
        }

        if set.len() > 1 {
            panic!("Found set larger then 1");
        }

        remove_rules.push(set.iter().next().unwrap().clone());
    }

    let rule_to_field = sorted_rules_order
        .iter()
        .map(|(idx, s)| (rules[*s.iter().next().unwrap()].name.clone(), idx))
        .collect::<HashMap<_, _>>();

    let my_ticket_product: u64 = rule_to_field.iter().filter(|(k,v)| k.starts_with("departure")).map(|(_,v)| my_ticket[**v]).product();
    dbg!(&my_ticket_product);
}
