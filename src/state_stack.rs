#[derive(Debug)]
//TODO: replace game_over with GameOver state
pub enum StateType {
    Playing,
    Menu
}

#[derive(Debug)]
pub struct StateStack {
    states: Vec<StateType>
}

impl StateStack {
    pub fn new() -> Self {
        StateStack {
            states: Vec::new()
        }
    }
    pub fn pop(&mut self) {
        self.states.pop();
    }
    pub fn push(&mut self, state: StateType) {
        self.states.push(state);
    }
    pub fn top(&self) -> Option<&StateType> {
        match self.states.len() {
            0 => None,
            _ => Some(&self.states[self.states.len() - 1])
        }
    }
}
