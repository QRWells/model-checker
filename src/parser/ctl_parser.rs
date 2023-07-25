use once_cell::sync::OnceCell;
use pest::{iterators::Pairs, pratt_parser::PrattParser, Parser};

use crate::ctl::CTLFormulae;

#[derive(pest_derive::Parser)]
#[grammar = "parser/ctl.pest"]
pub struct CTLParser;

fn ctl_parser() -> &'static PrattParser<Rule> {
    static INSTANCE: OnceCell<PrattParser<Rule>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        // Precedence is defined lowest to highest
        PrattParser::new()
            // Addition and subtract have equal precedence
            .op(Op::infix(And, Left) | Op::infix(Or, Left) | Op::infix(Implies, Left))
            .op(Op::infix(Until, Right) | Op::infix(WeakUntil, Right) | Op::infix(Release, Right))
            .op(Op::prefix(All) | Op::prefix(Exists))
            .op(Op::prefix(Not) | Op::prefix(Next) | Op::prefix(Finally) | Op::prefix(Globally))
    })
}

pub fn parse_ctl(input: &str) -> Result<CTLFormulae, Box<pest::error::Error<Rule>>> {
    match CTLParser::parse(Rule::formula, input) {
        Ok(mut pairs) => Ok(parse_expr(pairs.next().unwrap().into_inner())),
        Err(e) => {
            eprintln!("Parse failed: {:?}", e);
            Err(Box::new(e))
        }
    }
}

fn parse_expr(pairs: Pairs<Rule>) -> CTLFormulae {
    ctl_parser()
        .map_primary(|primary| match primary.as_rule() {
            Rule::AP => CTLFormulae::Atomic(primary.as_str().to_owned()),
            Rule::TRUE => CTLFormulae::True,
            Rule::formula => parse_expr(primary.into_inner()),
            rule => unreachable!("Expr::parse expected atom, found {:?}", rule),
        })
        .map_infix(|lhs, op, rhs| match op.as_rule() {
            Rule::And => CTLFormulae::And(Box::new(lhs), Box::new(rhs)),
            Rule::Or => CTLFormulae::Or(Box::new(lhs), Box::new(rhs)),
            Rule::Implies => {
                CTLFormulae::Or(Box::new(CTLFormulae::Not(Box::new(lhs))), Box::new(rhs))
            }
            Rule::Until => CTLFormulae::Until(Box::new(lhs), Box::new(rhs)),
            Rule::Release => CTLFormulae::Release(Box::new(lhs), Box::new(rhs)),
            Rule::WeakUntil => todo!(),
            _ => unreachable!(),
        })
        .map_prefix(|op, rhs| match op.as_rule() {
            Rule::All => match rhs {
                CTLFormulae::All(_) => {
                    eprintln!("Warning: repeated quantifier");
                    rhs
                }
                CTLFormulae::Exist(_) => {
                    panic!("Cannot mix quantifiers");
                }
                _ => CTLFormulae::All(Box::new(rhs)),
            },
            Rule::Exists => match rhs {
                CTLFormulae::Exist(_) => {
                    eprintln!("Warning: repeated quantifier");
                    rhs
                }
                CTLFormulae::All(_) => {
                    panic!("Cannot mix quantifiers");
                }
                CTLFormulae::Next(_)
                | CTLFormulae::Until(_, _)
                | CTLFormulae::Globally(_)
                | CTLFormulae::Release(_, _) => CTLFormulae::Exist(Box::new(rhs)),
                _ => {
                    panic!("Path quantifier must be followed by a temporal operator")
                }
            },
            Rule::Next => CTLFormulae::Next(Box::new(rhs)),
            Rule::Not => CTLFormulae::Not(Box::new(rhs)),
            Rule::Finally => CTLFormulae::Finally(Box::new(rhs)),
            Rule::Globally => CTLFormulae::Globally(Box::new(rhs)),
            _ => unreachable!(),
        })
        .parse(pairs)
}
