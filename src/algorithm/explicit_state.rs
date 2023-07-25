use std::collections::HashSet;

use crate::{
    ctl::{to_normal_form, CTLFormulae},
    model::kripke::Kripke,
};

pub fn explicit_state_checking(model: &mut Kripke, formula: CTLFormulae) {
    let normal = to_normal_form(formula);
    println!("Normal form: {}", normal);
    process(model, &normal);
}

/// Process the formulae and return the index of the label
///
/// `usize::MAX` means true, which holds for all states
fn process(model: &mut Kripke, f: &CTLFormulae) -> usize {
    if let Some(id) = model.contains_label(&f.get_str()) {
        return id;
    }

    match f {
        CTLFormulae::True => usize::MAX,
        CTLFormulae::Atomic(atomic) => {
            if let Some(id) = model.contains_label(atomic) {
                id
            } else {
                panic!("Atomic formulae {} is not defined", atomic)
            }
        }
        CTLFormulae::Not(f) => {
            let not_id = model.get_label_id_or_add(&format!("!{}", f.get_str()));
            check_not(model, not_id, f);
            not_id
        }
        CTLFormulae::Or(f1, f2) => {
            let or_id = model.get_label_id_or_add(&format!("{}||{}", f1.get_str(), f2.get_str()));
            check_or(model, or_id, f1, f2);
            or_id
        }
        CTLFormulae::And(f1, f2) => {
            let and_id = model.get_label_id_or_add(&format!("{}&&{}", f1.get_str(), f2.get_str()));
            check_and(model, and_id, f1, f2);
            and_id
        }
        CTLFormulae::Exist(f) => match &**f {
            CTLFormulae::Next(n) => {
                let en_id = model.get_label_id_or_add(&format!("E{}", f.get_str()));
                check_exist_next(model, en_id, n);
                en_id
            }
            CTLFormulae::Globally(g) => {
                let eg_id = model.get_label_id_or_add(&format!("E{}", f.get_str()));
                check_exist_globally(model, eg_id, g);
                eg_id
            }
            CTLFormulae::Until(f1, f2) => {
                let eu_id = model.get_label_id_or_add(&format!("E{}", f.get_str()));
                check_exist_until(model, eu_id, f1, f2);
                eu_id
            }
            _ => {
                let en_id = model.get_label_id_or_add(&format!("E{}", f.get_str()));
                check_exist_next(model, en_id, f);
                en_id
            }
        },
        _ => {
            panic!("Not in normal form {}", f.get_str());
        }
    }
}

fn check_and(model: &mut Kripke, and_id: usize, f1: &CTLFormulae, f2: &CTLFormulae) {
    let f1_idx = process(model, f1);
    let f2_idx = process(model, f2);
    let s1 = model.get_state_with_label_as_set(f1_idx);
    let s2 = model.get_state_with_label_as_set(f2_idx);

    let intersection = s1.intersection(&s2).cloned().collect::<Vec<usize>>();
    for s in intersection {
        model.add_state_for_label(and_id, s);
    }
}

fn check_not(model: &mut Kripke, not_id: usize, f: &CTLFormulae) {
    let f_idx = process(model, f);
    let s_prime = model.get_state_with_label(f_idx);
    // get complement of s_prime
    let mut s_prime_complement = HashSet::new();
    model.states.clone().into_iter().for_each(|s| {
        if !s_prime.contains(&s.0) {
            s_prime_complement.insert(s.0);
        }
    });
    for s in s_prime_complement {
        model.add_state_for_label(not_id, s);
    }
}

fn check_or(model: &mut Kripke, or_id: usize, f1: &CTLFormulae, f2: &CTLFormulae) {
    let f1_idx = process(model, f1);
    let f2_idx = process(model, f2);
    let s1 = model.get_state_with_label_as_set(f1_idx);
    let s2 = model.get_state_with_label_as_set(f2_idx);

    // union s1 and s2
    let union = s1.union(&s2).cloned().collect::<Vec<usize>>();
    for s in union {
        model.add_state_for_label(or_id, s);
    }
}

fn check_exist_next(model: &mut Kripke, en_id: usize, f: &CTLFormulae) {
    // label every state that has a successor that satisfies f
    let f_idx = process(model, f);
    let states = model.get_state_with_label(f_idx);
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

    let mut T = model.get_state_with_label(f2_idx);
    for i in &T {
        model.add_state_for_label(eu_id, *i);
    }
    while let Some(s) = T.pop() {
        for t in &model.transitable_to(s) {
            let label_t = model.state_to_labels.get(t).unwrap();
            let condition2 = if f1_idx == usize::MAX {
                true
            } else {
                label_t.contains(&f1_idx)
            };
            if !label_t.contains(&eu_id) && condition2 {
                model.add_state_for_label(eu_id, *t);
                T.push(*t);
            }
        }
    }
}

fn check_exist_globally(model: &mut Kripke, eg_id: usize, f: &CTLFormulae) {
    let f_idx = process(model, f);
    let s_prime = model.get_state_with_label(f_idx);
    let sccs = model.non_trivial_scc_of(&s_prime);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_explicit() {
        let data = r#"
    {
        "states": [
            {
                "id": 0,
                "name": "s0",
                "labels": ["a"],
                "transit_to": [1, 2]
            }
        ],
        "initial_state": 0
    }"#;
        let k = Kripke::from_json(data).unwrap();
        println!("{:?}", k);
    }
}