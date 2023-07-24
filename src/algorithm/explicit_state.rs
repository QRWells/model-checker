use std::collections::HashSet;

use crate::{ctl::CTLFormulae, model::kripke::Kripke};

pub fn explicit_state_checking(model: &Kripke, formula: &CTLFormulae) -> Result<(), ()> {
    todo!()
}

fn process(model: &mut Kripke, f: &CTLFormulae) -> usize {
    if let Some(id) = model.contains_label(&f.get_str()) {
        id
    } else {
        0
    }
}

fn check_exist_next(model: &mut Kripke, en_id: usize, f: &CTLFormulae) {
    // label every state that has a successor that satisfies f
    let f_idx = process(model, f);
    let states = model.label_to_states.get(&f_idx).unwrap().clone();
    let transitions = model.transitions.clone();
    for (s, t) in transitions {
        // if there is an intersection between states and t
        if states.iter().any(|i| t.contains(i)) {
            model.add_state_for_label(en_id, s);
        }
    }
}

fn check_exist_until(model: &mut Kripke, eu_id: usize, f1: &CTLFormulae, f2: &CTLFormulae) {
    let f1_idx = process(model, f1);
    let f2_idx = process(model, f2);

    let mut T = model
        .label_to_states
        .get(&f2_idx)
        .unwrap()
        .iter()
        .cloned()
        .collect::<Vec<usize>>();
    for i in &T {
        model.add_state_for_label(eu_id, *i);
    }
    while let Some(s) = T.pop() {
        for t in &model.transitable_to(s) {
            let label_t = model.state_to_labels.get(t).unwrap();
            if label_t.contains(&eu_id) || !label_t.contains(&f1_idx) {
                continue;
            }
            model.add_state_for_label(eu_id, *t);
            T.push(*t);
        }
    }
}

fn check_exist_globally(model: &mut Kripke, eg_id: usize, f: &CTLFormulae) {
    let f_idx = process(model, f);
    let s_prime = model.label_to_states.get(&f_idx).unwrap().clone();
    let sccs = model.non_trivial_scc();
    // union sccs into one set
    let mut sccs_set = HashSet::new();
    for scc in &sccs {
        sccs_set.extend(scc);
    }
    let mut sccs_set = sccs_set.into_iter().collect::<Vec<usize>>();

    for s in &sccs_set {
        model.add_state_for_label(eg_id, *s);
    }

    while let Some(s) = sccs_set.pop() {
        // for all t such that t in S_prime and t -> s
        for t in &model.transitable_to(s) {
            if !s_prime.contains(t) {
                continue;
            }
            let label_t = model.state_to_labels.get(t).unwrap();
            if !label_t.contains(&eg_id) {
                model.add_state_for_label(eg_id, *t);
                sccs_set.push(*t);
            }
        }
    }
}
