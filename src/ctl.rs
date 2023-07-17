#[derive(Debug, Clone)]
pub enum CTLFormulae {
    True,
    Atomic(String),
    Not(Box<CTLFormulae>),
    And(Box<CTLFormulae>, Box<CTLFormulae>),
    All(Box<CTLFormulae>),
    Exist(Box<CTLFormulae>),
    Next(Box<CTLFormulae>),
    Until(Box<CTLFormulae>, Box<CTLFormulae>),
}
