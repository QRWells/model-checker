use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum LTLFormulae {
    True,
    Atomic(String),
    And(Box<LTLFormulae>, Box<LTLFormulae>),
    Not(Box<LTLFormulae>),
    Next(Box<LTLFormulae>),
    Until(Box<LTLFormulae>, Box<LTLFormulae>),
}

impl Display for LTLFormulae {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LTLFormulae::True => write!(formatter, "true"),
            LTLFormulae::Atomic(s) => write!(formatter, "{}", s),
            LTLFormulae::Not(f) => write!(formatter, "¬{}", f),
            LTLFormulae::And(f, g) => write!(formatter, "({} ∧ {})", f, g),
            LTLFormulae::Next(f) => write!(formatter, "X{}", f),
            LTLFormulae::Until(f, g) => write!(formatter, "({} U {})", f, g),
        }
    }
}
