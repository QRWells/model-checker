#[derive(Debug, Clone)]
pub enum LTLFormulae {
    True,
    Atomic(String),
    And(Box<LTLFormulae>, Box<LTLFormulae>),
    Not(Box<LTLFormulae>),
    Next(Box<LTLFormulae>),
    Until(Box<LTLFormulae>, Box<LTLFormulae>),
}
