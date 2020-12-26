use std::{num::ParseIntError, str::FromStr};

type Operand = u64;
#[derive(Debug)]
enum Expr {
    Leaf(Operand),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

impl FromStr for Expr {
    type Err = ParseIntError;

    fn from_str(mut s: &str) -> Result<Self, Self::Err> {
        let parse_operand = |s: &str| -> Result<(usize, Self), Self::Err> {
            if s.starts_with('(') {
                let subexpr_end = s
                    .char_indices()
                    .skip(1)
                    .scan(1, |par_count, (offset, ch)| {
                        if *par_count == 0 {
                            return None;
                        }
                        match ch {
                            '(' => *par_count += 1,
                            ')' => *par_count -= 1,
                            _ => (),
                        }
                        Some((offset, ch))
                    })
                    .last()
                    .map(|(offset, _)| offset)
                    .unwrap();
                s['('.len_utf8()..subexpr_end]
                    .parse()
                    .map(|expr| (subexpr_end + ')'.len_utf8(), expr))
            } else {
                let operand_end = s
                    .char_indices()
                    .find_map(|(offset, ch)| Some(offset).filter(|_| ch == ' '))
                    .unwrap_or(s.len());
                s[..operand_end]
                    .parse()
                    .map(|op| (operand_end, Expr::Leaf(op)))
            }
        };

        let (offset, mut expr) = parse_operand(s)?;
        s = &s[offset..];

        while !s.is_empty() {
            let (offset, outer_expr) = if let Some(t) = s.strip_prefix(" + ") {
                let (offset, rhs) = parse_operand(t)?;
                (
                    " + ".len() + offset,
                    Expr::Add(Box::new(expr), Box::new(rhs)),
                )
            } else {
                let t = s.strip_prefix(" * ").unwrap();
                let (offset, rhs) = parse_operand(t)?;
                (
                    " * ".len() + offset,
                    Expr::Mul(Box::new(expr), Box::new(rhs)),
                )
            };
            s = &s[offset..];
            expr = outer_expr;
        }
        Ok(expr)
    }
}

impl Expr {
    fn eval(&self) -> Operand {
        match self {
            Expr::Leaf(v) => *v,
            Expr::Add(a, b) => a.eval() + b.eval(),
            Expr::Mul(a, b) => a.eval() * b.eval(),
        }
    }
}

fn main() {
    let answer: Operand = String::from_utf8(std::fs::read("input/day18").unwrap())
        .unwrap()
        .split_terminator('\n')
        .map(|expr| expr.parse::<Expr>().unwrap().eval())
        .sum();
    println!("{}", answer);
}
