use std::collections::{HashMap, HashSet};

use petgraph::graph::{DiGraph, NodeIndex};
use serde::{Deserialize, Serialize};

/// Kripke structure.
#[derive(Debug, Serialize)]
pub struct Kripke {
    /// The states.
    ///
    /// key: id, value: state
    pub states: HashMap<usize, String>,

    /// The transitions.
    ///
    /// key: from, value: to
    pub transitions: HashMap<usize, HashSet<usize>>,

    /// The labels.
    ///
    /// key: label, value: name
    pub labels: HashMap<usize, String>,

    /// The mapping from label to states.
    ///
    /// key: label, value: states
    pub label_to_states: HashMap<usize, HashSet<usize>>,

    /// The mapping from state to labels.
    ///
    /// key: state, value: labels
    pub state_to_labels: HashMap<usize, HashSet<usize>>,

    /// The initial state.
    pub initial_state: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct StateInfo {
    id: usize,
    name: String,
    labels: Vec<String>,
    transit_to: Vec<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct KripkeBuilder {
    states: Vec<StateInfo>,
    initial_state: usize,
}

impl Kripke {
    pub fn from_json(data: &str) -> Result<Kripke, serde_json::Error> {
        let builder = serde_json::from_str::<KripkeBuilder>(data);
        if let Ok(k) = builder {
            let init = k.initial_state;
            let mut states = HashMap::new();
            let mut transitions = HashMap::new();
            let mut labels = HashMap::new();
            let mut label_to_states: HashMap<usize, HashSet<usize>> = HashMap::new();
            let mut state_to_labels: HashMap<usize, HashSet<usize>> = HashMap::new();
            let mut temp_labels: HashMap<String, usize> = HashMap::new();

            for state in k.states {
                states.insert(state.id, state.name);
                state_to_labels.insert(state.id, HashSet::new());
                transitions.insert(state.id, state.transit_to.into_iter().collect());
                for label in state.labels {
                    if let Some(id) = temp_labels.get(&label) {
                        label_to_states.get_mut(id).unwrap().insert(state.id);
                        state_to_labels.get_mut(&state.id).unwrap().insert(*id);
                    } else {
                        let id = labels.len();
                        temp_labels.insert(label.clone(), id);
                        labels.insert(id, label);
                        label_to_states.insert(id, {
                            let mut set = HashSet::new();
                            set.insert(state.id);
                            set
                        });
                        state_to_labels.insert(state.id, {
                            let mut set = HashSet::new();
                            set.insert(id);
                            set
                        });
                    }
                }
            }

            // TODO: check if there is non-exist state in transitions
            // TODO: check if there is orphan state in transitions

            Ok(Kripke {
                states,
                transitions,
                labels,
                label_to_states,
                state_to_labels,
                initial_state: init,
            })
        } else {
            Err(builder.unwrap_err())
        }
    }

    pub fn contains_label(&self, label: &str) -> Option<usize> {
        self.labels
            .iter()
            .find_map(|(id, name)| if name == label { Some(*id) } else { None })
    }

    pub fn get_label_id_or_add(&mut self, label: &str) -> usize {
        if let Some(id) = self.contains_label(label) {
            id
        } else {
            let id = self.labels.len();
            self.labels.insert(id, label.to_string());
            id
        }
    }

    pub fn get_state_with_label(&self, label_index: usize) -> Vec<usize> {
        if label_index == usize::MAX {
            // true
            return self.states.keys().cloned().collect();
        } else {
            self.label_to_states
                .get(&label_index)
                .unwrap_or(&HashSet::new())
                .iter()
                .cloned()
                .collect()
        }
    }

    pub fn get_state_with_label_as_set(&self, label_index: usize) -> HashSet<usize> {
        if label_index == usize::MAX {
            // true
            return self.states.keys().cloned().collect();
        } else {
            self.label_to_states
                .get(&label_index)
                .unwrap_or(&HashSet::new())
                .clone()
        }
    }

    pub fn add_state_for_label(&mut self, label: usize, state: usize) {
        if let Some(states) = self.label_to_states.get_mut(&label) {
            states.insert(state);
        } else {
            self.label_to_states.insert(label, {
                let mut set = HashSet::new();
                set.insert(state);
                set
            });
        }

        if let Some(labels) = self.state_to_labels.get_mut(&state) {
            labels.insert(label);
        } else {
            self.state_to_labels.insert(state, {
                let mut set = HashSet::new();
                set.insert(label);
                set
            });
        }
    }

    pub fn transitable_to(&self, state: usize) -> Vec<usize> {
        self.transitions
            .iter()
            .filter_map(|(from, to)| {
                if to.contains(&state) {
                    Some(*from)
                } else {
                    None
                }
            })
            .collect::<Vec<usize>>()
    }

    pub fn to_graph(&self) -> DiGraph<usize, ()> {
        DiGraph::<usize, ()>::from_edges(self.transitions.iter().flat_map(|(from, to)| {
            to.iter()
                .map(move |t| (NodeIndex::new(*from), NodeIndex::new(*t)))
        }))
    }

    pub fn non_trivial_scc(&self) -> Vec<Vec<usize>> {
        let g = self.to_graph();
        let sccs = petgraph::algo::tarjan_scc(&g);
        sccs.into_iter()
            .filter(|scc| {
                scc.len() > 1 // more than one node
                    || self
                        .transitions
                        .iter()
                        .any(|(from, to)| from == &scc[0].index() && to.contains(&scc[0].index()))
                // self loop
            })
            .map(|scc| {
                scc.into_iter()
                    .map(|i| *g.node_weight(i).unwrap())
                    .collect()
            })
            .collect()
    }

    pub fn non_trivial_scc_of(&self, states: &[usize]) -> Vec<Vec<usize>> {
        // retain transitions that are in states
        let transitions = self
            .transitions
            .iter()
            .filter(|(from, to)| states.contains(from) && to.iter().all(|t| states.contains(t)))
            .map(|(from, to)| (*from, to.clone()))
            .collect::<HashMap<usize, HashSet<usize>>>();

        let transitions_vec = transitions
            .iter()
            .flat_map(|(from, to)| to.iter().map(move |t| (*from, *t)))
            .collect::<Vec<(usize, usize)>>();

        let g = construct_graph(states, &transitions_vec);

        let sccs = petgraph::algo::tarjan_scc(&g);

        sccs.into_iter()
            .filter(|scc| scc.len() > 1)
            .map(|scc| {
                scc.into_iter()
                    .map(|i| *g.node_weight(i).unwrap())
                    .collect()
            })
            .collect()
    }
}

fn construct_graph(states: &[usize], transitions: &[(usize, usize)]) -> DiGraph<usize, ()> {
    let mut g = DiGraph::<usize, ()>::new();
    let mut nodes = HashMap::new();

    for state in states {
        nodes.insert(*state, g.add_node(*state));
    }

    for (from, to) in transitions {
        g.add_edge(*nodes.get(from).unwrap(), *nodes.get(to).unwrap(), ());
    }

    g
}

#[cfg(test)]
mod tests {

    use crate::{algorithm::explicit_state::explicit_state_checking, parser::ctl_parser::parse_ctl};

    use super::*;

    #[test]
    fn test_kripke() {
        let data = r#"
        {
            "states": [
                {
                    "id": 1,
                    "name": "s1",
                    "labels": [],
                    "transit_to": [
                        2,
                        3
                    ]
                },
                {
                    "id": 2,
                    "name": "s2",
                    "labels": [
                        "s",
                        "e"
                    ],
                    "transit_to": [
                        5
                    ]
                },
                {
                    "id": 3,
                    "name": "s3",
                    "labels": [
                        "c"
                    ],
                    "transit_to": [
                        1,
                        6
                    ]
                },
                {
                    "id": 4,
                    "name": "s4",
                    "labels": [
                        "c",
                        "h"
                    ],
                    "transit_to": [
                        1,
                        3,
                        4
                    ]
                },
                {
                    "id": 5,
                    "name": "s5",
                    "labels": [
                        "s",
                        "c",
                        "e"
                    ],
                    "transit_to": [
                        2,
                        3
                    ]
                },
                {
                    "id": 6,
                    "name": "s6",
                    "labels": [
                        "s",
                        "c"
                    ],
                    "transit_to": [
                        7
                    ]
                },
                {
                    "id": 7,
                    "name": "s7",
                    "labels": [
                        "s",
                        "c",
                        "h"
                    ],
                    "transit_to": [
                        4
                    ]
                }
            ],
            "initial_state": 0
        }"#;
        let res = Kripke::from_json(data);
        if let Ok(mut k) = res {
            let formula = parse_ctl("AG(s -> AF h)");
            if let Ok(formula) = formula {
                explicit_state_checking(&mut k, formula.clone());

                let labels = k.labels;
                let mut labels: Vec<(usize, String)> = labels.into_iter().collect();
                labels.sort_by(|a, b| a.0.cmp(&b.0));
                println!("{:?}", labels);

                let states = k.label_to_states;
                let mut states: Vec<(usize, HashSet<usize>)> = states.into_iter().collect();
                states.sort_by(|a, b| a.0.cmp(&b.0));
                println!("{:?}", states);
            } else {
                println!("Error: {}", formula.err().unwrap());
            }
        } else {
            println!("Error: {}", res.err().unwrap());
        }
    }
}
