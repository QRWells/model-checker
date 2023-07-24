pub mod explicit_state;

use crate::{ltl::LTLFormulae, model::kripke::Kripke};

pub fn ltl_checking(model: &Kripke, formula: &LTLFormulae) -> Result<(), ()> {
    todo!()
}

pub fn ctl_checking(model: &Kripke, formula: &LTLFormulae) -> Result<(), ()> {
    todo!()
}
