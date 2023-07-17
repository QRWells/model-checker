use once_cell::sync::OnceCell;
use pest::{iterators::Pairs, pratt_parser::PrattParser, Parser};

use crate::ltl::LTLFormulae;

#[derive(pest_derive::Parser)]
#[grammar = "parser/ltl.pest"]
pub struct LTLParser;

fn ltl_parser() -> &'static PrattParser<Rule> {
    static INSTANCE: OnceCell<PrattParser<Rule>> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        use pest::pratt_parser::{Assoc::*, Op};
        use Rule::*;

        // Precedence is defined lowest to highest
        PrattParser::new()
            // Addition and subtract have equal precedence
            .op(Op::infix(And, Left) | Op::infix(Or, Left) | Op::infix(Implies, Left))
            .op(Op::infix(Until, Right) | Op::infix(WeakUntil, Right) | Op::infix(Release, Right))
            .op(Op::prefix(Globally) | Op::prefix(Eventually))
            .op(Op::prefix(Not) | Op::prefix(Next))
    })
}

pub fn parse_ltl(input: &str) -> Result<LTLFormulae, Box<pest::error::Error<Rule>>> {
    match LTLParser::parse(Rule::formula, input) {
        Ok(mut pairs) => Ok(parse_expr(pairs.next().unwrap().into_inner())),
        Err(e) => {
            eprintln!("Parse failed: {:?}", e);
            Err(Box::new(e))
        }
    }
}

fn parse_expr(pairs: Pairs<Rule>) -> LTLFormulae {
    ltl_parser()
        .map_primary(|primary| match primary.as_rule() {
            Rule::AP => LTLFormulae::Atomic(primary.as_str().to_owned()),
            Rule::TRUE => LTLFormulae::True,
            Rule::formula => parse_expr(primary.into_inner()),
            rule => unreachable!("Expr::parse expected atom, found {:?}", rule),
        })
        .map_infix(|lhs, op, rhs| match op.as_rule() {
            Rule::And => LTLFormulae::And(Box::new(lhs), Box::new(rhs)),
            Rule::Or => LTLFormulae::And(
                Box::new(LTLFormulae::Not(Box::new(lhs))),
                Box::new(LTLFormulae::Not(Box::new(rhs))),
            ),
            Rule::Implies => {
                LTLFormulae::And(Box::new(lhs), Box::new(LTLFormulae::Not(Box::new(rhs))))
            }
            Rule::Until => LTLFormulae::Until(Box::new(lhs), Box::new(rhs)),
            Rule::Release => todo!(),
            Rule::WeakUntil => todo!(),
            _ => unreachable!(),
        })
        .map_prefix(|op, rhs| match op.as_rule() {
            Rule::Globally => LTLFormulae::Not(Box::new(LTLFormulae::Until(
                Box::new(LTLFormulae::True),
                Box::new(LTLFormulae::Not(Box::new(rhs))),
            ))),
            Rule::Eventually => LTLFormulae::Until(Box::new(LTLFormulae::True), Box::new(rhs)),
            Rule::Next => LTLFormulae::Next(Box::new(rhs)),
            Rule::Not => LTLFormulae::Not(Box::new(rhs)),
            _ => unreachable!(),
        })
        .parse(pairs)
}
