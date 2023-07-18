use serde::{Serialize, Deserialize};

pub mod kripke;

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

pub trait Model {
    fn get_states(&self) -> &Vec<State>;
    fn get_transitions(&self) -> &Vec<Transition>;
    fn get_labels(&self) -> &Vec<Label>;
    fn get_initial_state(&self) -> usize;
}
