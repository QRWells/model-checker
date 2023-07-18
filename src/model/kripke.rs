use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kripke {
    pub states: Vec<State>,
    pub transitions: Vec<Transition>,
    pub labels: Vec<Label>,
    pub initial_state: usize,
}

impl Model for Kripke {
    fn get_states(&self) -> &Vec<State> {
        &self.states
    }

    fn get_transitions(&self) -> &Vec<Transition> {
        &self.transitions
    }

    fn get_labels(&self) -> &Vec<Label> {
        &self.labels
    }

    fn get_initial_state(&self) -> usize {
        self.initial_state
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
                "name": "s0"
            }
        ],
        "transitions": [
            [0, 0],
            [0, 1]
        ],
        "labels": [
            {
                "state": 0,
                "labels": ["a"]
            }
        ],
        "initial_state": 0
    }"#;
        let k: Kripke = serde_json::from_str(data).unwrap();
        println!("{:?}", k);
    }
}
