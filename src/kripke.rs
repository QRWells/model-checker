use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kripke {
    pub states: Vec<State>,
    pub transitions: Vec<Transition>,
    pub labels: Vec<Label>,
    pub initial_state: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    pub id: usize,
    pub name: String,
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
pub type Transition = (usize, usize);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Label {
    pub state: usize,
    pub labels: Vec<String>,
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
