use std::collections::HashMap;

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
    pub transitions: HashMap<usize, Vec<usize>>,

    /// The labels.
    ///
    /// key: label, value: name
    pub labels: HashMap<usize, String>,

    /// The mapping from label to states.
    ///
    /// key: label, value: states
    pub label_map: HashMap<usize, Vec<usize>>,

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
            let mut label_map: HashMap<usize, Vec<usize>> = HashMap::new();

            let mut temp_labels: HashMap<String, usize> = HashMap::new();

            for state in k.states {
                states.insert(state.id, state.name);
                transitions.insert(state.id, state.transit_to);
                for label in state.labels {
                    if let Some(id) = temp_labels.get(&label) {
                        label_map.get_mut(id).unwrap().push(state.id);
                    } else {
                        let id = labels.len();
                        temp_labels.insert(label.clone(), id);
                        labels.insert(id, label);
                        label_map.insert(id, vec![state.id]);
                    }
                }
            }

            // TODO: check if there is non-exist state in transitions
            // TODO: check if there is orphan state in transitions

            Ok(Kripke {
                states,
                transitions,
                labels,
                label_map,
                initial_state: init,
            })
        } else {
            Err(builder.unwrap_err())
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_kripke() {
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
