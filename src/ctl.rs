use std::fmt::Display;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub enum CTLFormulae {
    True,
    Atomic(String),

    Not(Box<CTLFormulae>),
    And(Box<CTLFormulae>, Box<CTLFormulae>),
    Or(Box<CTLFormulae>, Box<CTLFormulae>),

    All(Box<CTLFormulae>),
    Exist(Box<CTLFormulae>),

    Next(Box<CTLFormulae>),
    Finally(Box<CTLFormulae>),
    Globally(Box<CTLFormulae>),
    Until(Box<CTLFormulae>, Box<CTLFormulae>),
    Release(Box<CTLFormulae>, Box<CTLFormulae>),
}

impl Display for CTLFormulae {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CTLFormulae::True => write!(formatter, "true"),
            CTLFormulae::Atomic(s) => write!(formatter, "{}", s),
            CTLFormulae::Not(f) => write!(formatter, "¬{}", f),
            CTLFormulae::And(f, g) => write!(formatter, "({} ∧ {})", f, g),
            CTLFormulae::Or(f, g) => write!(formatter, "({} ∨ {})", f, g),
            CTLFormulae::All(f) => write!(formatter, "A{}", f),
            CTLFormulae::Exist(f) => write!(formatter, "E{}", f),
            CTLFormulae::Next(f) => write!(formatter, "X{}", f),
            CTLFormulae::Finally(f) => write!(formatter, "F{}", f),
            CTLFormulae::Globally(f) => write!(formatter, "G{}", f),
            CTLFormulae::Until(f, g) => write!(formatter, "({} U {})", f, g),
            CTLFormulae::Release(f, g) => write!(formatter, "({} R {})", f, g),
        }
    }
}

impl CTLFormulae {
    pub fn get_str(&self) -> String {
        match self {
            CTLFormulae::True => "true".to_string(),
            CTLFormulae::Atomic(s) => s.clone(),
            CTLFormulae::Not(f) => format!("!{}", f.get_str()),
            CTLFormulae::And(f, g) => format!("({}&&{})", f.get_str(), g.get_str()),
            CTLFormulae::Or(f, g) => format!("({}||{})", f.get_str(), g.get_str()),
            CTLFormulae::All(f) => format!("A{}", f.get_str()),
            CTLFormulae::Exist(f) => format!("E{}", f.get_str()),
            CTLFormulae::Next(f) => format!("X{}", f.get_str()),
            CTLFormulae::Finally(f) => format!("F{}", f.get_str()),
            CTLFormulae::Globally(f) => format!("G{}", f.get_str()),
            CTLFormulae::Until(f, g) => format!("({}U{})", f.get_str(), g.get_str()),
            CTLFormulae::Release(f, g) => format!("({}R{})", f.get_str(), g.get_str()),
        }
    }
}

pub fn to_normal_form_rec(formulae: CTLFormulae) -> CTLFormulae {
    match formulae {
        CTLFormulae::True | CTLFormulae::Atomic(_) => formulae,
        CTLFormulae::And(f, g) => {
            let f = Box::new(to_normal_form_rec(*f));
            let g = Box::new(to_normal_form_rec(*g));
            CTLFormulae::And(f, g)
        }
        CTLFormulae::Or(f, g) => {
            let f = Box::new(to_normal_form_rec(*f));
            let g = Box::new(to_normal_form_rec(*g));
            CTLFormulae::Or(f, g)
        }
        CTLFormulae::All(all) => match *all {
            CTLFormulae::Next(f) => CTLFormulae::Not(Box::new(CTLFormulae::Exist(Box::new(
                CTLFormulae::Next(Box::new(CTLFormulae::Not(Box::new(to_normal_form_rec(*f))))),
            )))),
            CTLFormulae::Finally(f) => CTLFormulae::Not(Box::new(CTLFormulae::Exist(Box::new(
                CTLFormulae::Globally(Box::new(CTLFormulae::Not(Box::new(to_normal_form_rec(*f))))),
            )))),
            CTLFormulae::Globally(f) => CTLFormulae::Not(Box::new(CTLFormulae::Exist(Box::new(
                CTLFormulae::Finally(Box::new(CTLFormulae::Not(Box::new(to_normal_form_rec(*f))))),
            )))),
            CTLFormulae::Until(f, g) => {
                let f = Box::new(to_normal_form_rec(*f));
                let g = Box::new(to_normal_form_rec(*g));
                CTLFormulae::And(
                    Box::new(CTLFormulae::Not(Box::new(CTLFormulae::Exist(Box::new(
                        CTLFormulae::Until(
                            Box::new(CTLFormulae::Not(g.clone())),
                            Box::new(CTLFormulae::And(
                                Box::new(CTLFormulae::Not(f)),
                                Box::new(CTLFormulae::Not(g.clone())),
                            )),
                        ),
                    ))))),
                    Box::new(CTLFormulae::Not(Box::new(CTLFormulae::Exist(Box::new(
                        CTLFormulae::Globally(Box::new(CTLFormulae::Not(g))),
                    ))))),
                )
            }
            CTLFormulae::Release(f, g) => {
                let f = Box::new(to_normal_form_rec(*f));
                let g = Box::new(to_normal_form_rec(*g));

                CTLFormulae::Not(Box::new(CTLFormulae::Exist(Box::new(CTLFormulae::Until(
                    Box::new(CTLFormulae::Not(f)),
                    Box::new(CTLFormulae::Not(g)),
                )))))
            }
            _ => to_normal_form_rec(*all),
        },
        CTLFormulae::Exist(e) => match *e {
            CTLFormulae::Finally(f) => CTLFormulae::Exist(Box::new(CTLFormulae::Until(
                Box::new(CTLFormulae::True),
                Box::new(to_normal_form_rec(*f)),
            ))),
            CTLFormulae::Release(_, _) => todo!(),
            _ => CTLFormulae::Exist(Box::new(*e)),
        },
        CTLFormulae::Next(f) => {
            let f = Box::new(to_normal_form_rec(*f));
            CTLFormulae::Next(f)
        }
        CTLFormulae::Finally(f) => {
            let f = Box::new(to_normal_form_rec(*f));
            CTLFormulae::Finally(f)
        }
        CTLFormulae::Globally(f) => {
            let f = Box::new(to_normal_form_rec(*f));
            CTLFormulae::Globally(f)
        }
        CTLFormulae::Until(f, g) => {
            let f = Box::new(to_normal_form_rec(*f));
            let g = Box::new(to_normal_form_rec(*g));
            CTLFormulae::Until(f, g)
        }
        CTLFormulae::Release(f, g) => {
            let f = Box::new(to_normal_form_rec(*f));
            let g = Box::new(to_normal_form_rec(*g));
            CTLFormulae::Release(f, g)
        }
        CTLFormulae::Not(f) => {
            if let CTLFormulae::Not(f) = *f {
                to_normal_form_rec(*f)
            } else {
                let f = Box::new(to_normal_form_rec(*f));
                CTLFormulae::Not(f)
            }
        }
    }
}

pub fn to_normal_form(formulae: CTLFormulae) -> CTLFormulae {
    let pass1 = to_normal_form_rec(formulae);
    let pass2 = to_normal_form_rec(pass1);
    pass2
}
