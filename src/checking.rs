use crate::{ltl::LTLFormulae, model::Model};

pub fn ltl_checking<M: Model>(model: &M, formula: &LTLFormulae) -> Result<(), ()> {
    todo!()
}

pub fn ctl_checking<M: Model>(model: &M, formula: &LTLFormulae) -> Result<(), ()> {
    todo!()
}
