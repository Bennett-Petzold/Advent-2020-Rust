use std::str::FromStr;

use anyhow::{anyhow, bail};

#[derive(Debug, PartialEq)]
pub enum Operator {
    Plus,
    Mult,
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Value(isize),
    Op((Box<Self>, Operator, Box<Self>)),
    Group(Vec<Self>),
}

impl TryFrom<char> for Operator {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(Self::Plus),
            '*' => Ok(Self::Mult),
            _ => Err(anyhow!("Not a valid operator")),
        }
    }
}

impl FromStr for Expr {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let extra_space = s.replace('(', "( ").replace(')', " )");
        let mut parts = extra_space.split_whitespace();
        let mut group = Vec::new();
        println!("Evaluating: {extra_space}");

        while let Some(substr) = parts.next() {
            match substr {
                "+" | "*" => {
                    let prev = group.pop().ok_or(anyhow!("Missing left operand"))?;
                    let next =
                        Expr::from_str(parts.by_ref().)?;

                    group.push(Self::Op((
                        Box::new(prev),
                        Operator::try_from(substr.chars().next().unwrap())?,
                        Box::new(next),
                    )))
                }
                "(" => {
                    println!("PAREN");
                    let mut paren = String::new();
                    for inner_str in parts.by_ref() {
                        if inner_str == ")" {
                            break;
                        } else {
                            paren.push_str(inner_str);
                            paren.push(' ');
                        }
                    }
                    group.push(Self::from_str(&paren)?);
                }
                s if s.parse::<isize>().is_ok() => {
                    group.push(Self::Value(s.parse::<isize>().unwrap()))
                }
                s => bail!("Not a valid equation symbol: {}", s),
            }
        }
        Ok(Self::Group(group))
    }
}

impl Expr {
    pub fn eval(&self) -> anyhow::Result<isize> {
        match self {
            Self::Value(x) => Ok(*x),
            Self::Op((l, o, r)) => {
                let l = l.eval()?;
                let r = r.eval()?;
                match o {
                    Operator::Plus => Ok(l + r),
                    Operator::Mult => Ok(l * r),
                }
            }
            Self::Group(g) => Ok(0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        assert_eq!(
            Expr::from_str("1 + (2 * 3)").unwrap(),
            Expr::Group(vec![Expr::Op((
                Box::new(Expr::Value(1)),
                Operator::Plus,
                Box::new(Expr::Group(vec![Expr::Op((
                    Box::new(Expr::Value(2)),
                    Operator::Mult,
                    Box::new(Expr::Value(3))
                ))]))
            ))])
        )
    }

    #[test]
    fn calculate() {
        let pairs = [
            ("1 + 2 * 3 + 4 * 5 + 6", 71),
            ("1 + (2 * 3) + (4 * (5 + 6))", 51),
            ("2 * 3 + (4 * 5)", 26),
            ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437),
            ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240),
            ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13632),
        ];

        for (expr, value) in pairs {
            assert_eq!(Expr::from_str(expr).unwrap().eval().unwrap(), value);
        }
    }
}
