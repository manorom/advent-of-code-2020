use std::iter::Peekable;
use std::str::Chars;

type InputIter<'a> = Peekable<Chars<'a>>;

#[derive(Debug)]
enum ParseNode {
    Num(u64),
    Add {
        rhs: Box<ParseNode>,
        lhs: Box<ParseNode>,
    },
    Mul {
        lhs: Box<ParseNode>,
        rhs: Box<ParseNode>,
    },
}

impl ParseNode {
    fn parse_num(input_iter: &mut InputIter) -> ParseNode {
        let mut num_string = String::new();
        while input_iter
            .peek()
            .map(|i| *i != '+' && *i != ' ' && *i != '*' && *i != '(' && *i != ')')
            .unwrap_or(false)
        {
            num_string.push(input_iter.next().unwrap());
        }
        ParseNode::Num(num_string.parse::<u64>().unwrap())
    }

    fn parse_num_or_subexpr(input_iter: &mut InputIter) -> ParseNode {
        ParseNode::skip_whitespace(input_iter);
        println!("{}", input_iter.clone().collect::<String>());
        match input_iter.peek() {
            Some(c) if c.is_ascii_digit() => ParseNode::parse_num(input_iter),
            Some('(') => {
                input_iter.next();
                let lhs = ParseNode::parse_num_or_subexpr(input_iter);

                ParseNode::parse_remaining(lhs, input_iter, true)
            }
            Some(c) => {
                panic!("unexpected character {}", c)
            }
            None => {
                panic!("unexpected end of line")
            }
        }
    }

    fn parse_remaining(lhs: ParseNode, input_iter: &mut InputIter, subexpr: bool) -> ParseNode {
        ParseNode::skip_whitespace(input_iter);
        match input_iter.peek() {
            Some('+') => {
                input_iter.next();
                let rhs = ParseNode::parse_num_or_subexpr(input_iter);
                let add_node = ParseNode::Add {
                    lhs: lhs.into(),
                    rhs: rhs.into(),
                };
                ParseNode::parse_remaining(add_node, input_iter, subexpr)
            }
            Some('*') => {
                input_iter.next();
                let rhs = ParseNode::parse_expr(input_iter, subexpr);
                let n = ParseNode::Mul {
                    lhs: lhs.into(),
                    rhs: rhs.into(),
                };
                n
            }
            Some(')') => {
                if subexpr {
                    input_iter.next();
                    lhs
                } else {
                    panic!("unexpected ')'");
                }
            }
            Some(c) => {
                panic!("Unexpected character {}", c);
            }
            None => lhs,
        }
    }

    fn parse_expr(input_iter: &mut InputIter, subexpr: bool) -> ParseNode {
        ParseNode::skip_whitespace(input_iter);
        let lhs = ParseNode::parse_num_or_subexpr(input_iter);

        ParseNode::skip_whitespace(input_iter);
        if input_iter.peek().is_some() {
            ParseNode::parse_remaining(lhs, input_iter, subexpr)
        } else {
            lhs
        }
    }

    fn skip_whitespace(input_iter: &mut InputIter) {
        while let Some(' ') = input_iter.peek() {
            input_iter.next();
        }
    }
    fn evaluate(&self) -> u64 {
        match self {
            ParseNode::Num(i) => *i,
            ParseNode::Mul { lhs, rhs } => rhs.evaluate() * lhs.evaluate(),
            ParseNode::Add { lhs, rhs } => rhs.evaluate() + lhs.evaluate(),
        }
    }
}

enum Operator {
    No,
    Add,
    Mul,
}

impl Operator {
    fn exec(&self, accum: &mut u64, operand: u64) {
        use Operator::*;
        match self {
            Add => {
                *accum += operand;
            }
            Mul => {
                *accum *= operand;
            }
            No => {
                panic!("missing operator");
            }
        }
    }
}

fn parse_expr_part1(input_iter: &mut InputIter, subexpr: bool) -> u64 {
    let mut accum = 0;
    let mut next_operator = Operator::Add;
    while let Some(i) = input_iter.peek() {
        match *i {
            '0'..='9' => {
                let mut num_string = String::new();
                while input_iter
                    .peek()
                    .map(|i| *i != '+' && *i != ' ' && *i != '*' && *i != '(' && *i != ')')
                    .unwrap_or(false)
                {
                    num_string.push(input_iter.next().unwrap());
                }
                next_operator.exec(&mut accum, num_string.parse::<u64>().unwrap());
                next_operator = Operator::No;
            }
            '+' => {
                input_iter.next();
                next_operator = Operator::Add;
            }
            '*' => {
                input_iter.next();
                next_operator = Operator::Mul;
            }
            '(' => {
                input_iter.next();
                next_operator.exec(&mut accum, parse_expr_part1(input_iter, true));
                next_operator = Operator::No;
            }
            ')' => {
                if !subexpr {
                    panic!("Found ')' without matching '('");
                }
                input_iter.next();
                return accum;
            }
            ' ' => {
                input_iter.next();
            }
            _ => {
                panic!("Unknonw character {}", *i);
            }
        }
    }

    if subexpr {
        panic!("Found '(' witout closing ')'");
    }

    accum
}

fn main() {
    let input = include_str!("input.txt");
    let r = input
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| ParseNode::parse_expr(&mut l.chars().peekable(), false).evaluate())
        .sum::<u64>();
    dbg!(r);
    //dbg!(ParseNode::parse_expr(&mut "3 + (4 + 6) + 7 * 7".chars().peekable(), false));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part2() {
        assert_eq!(
            ParseNode::parse_expr(&mut "1 + (2 * 3) + (4 * (5 + 6))".chars().peekable(), false)
                .evaluate(),
            51
        );
        assert_eq!(
            ParseNode::parse_expr(&mut "2 * 3 + (4 * 5)".chars().peekable(), false).evaluate(),
            46
        );
        assert_eq!(
            ParseNode::parse_expr(&mut "5 + (8 * 3 + 9 + 3 * 4 * 3)".chars().peekable(), false)
                .evaluate(),
            1445
        );
        assert_eq!(
            ParseNode::parse_expr(
                &mut "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"
                    .chars()
                    .peekable(),
                false
            )
            .evaluate(),
            669060
        );
        assert_eq!(
            ParseNode::parse_expr(
                &mut "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"
                    .chars()
                    .peekable(),
                false
            )
            .evaluate(),
            23340
        );
    }
}
