

use std::collections::HashMap;

pub trait State {
    fn act(&mut self);
    fn transition_conditions(&self) -> Vec<TransitionOptions>;
}

pub trait StateWithTransitions: State + Clone {
    fn add_transitions(&mut self, condition: fn(&Self) -> TransitionOptions);
}
pub enum TransitionOptions {
    Stay,
    Change(usize),
    EndExecution
}


pub struct FSM {
    states: HashMap<usize, Box<dyn State>>,
    current: usize
}

pub struct ValidKey {
    k: usize
}

impl FSM {
    fn act(&mut self) {
        self.states.entry(self.current).and_modify(|state| state.act());
    }
    fn update_state(&mut self) -> bool {
        self.states
            .get(&self.current)
            .unwrap()
            .transition_conditions()
            .iter()
            .filter_map(|opt| match opt {
                TransitionOptions::Stay => None,
                TransitionOptions::Change(next) => {
                    self.current = *next;
                    Some(true)},
                TransitionOptions::EndExecution => Some(false),
            })
            .nth(1)
            .unwrap_or(true)
            ;
        false
    }

    pub fn execute(mut self) {
        while self.update_state() {
            self.act();
        }
    }

    pub fn new(initial_state: impl StateWithTransitions + 'static) -> (FSM, ValidKey) {
        let map = {
            let mut temp = HashMap::new();
            temp.insert(1 as usize, Box::new(initial_state) as Box<dyn State>);
            temp
        };
        (FSM { states: map, current: 1 }, ValidKey{ k: 1 })
    }

    pub fn insert_state(&mut self, state: impl StateWithTransitions + 'static) -> ValidKey {
        let next_key = self.states.len() + 1;
        self.states.insert(next_key, Box::new(state) as Box <dyn State>);
        ValidKey { k: next_key }
    }
}