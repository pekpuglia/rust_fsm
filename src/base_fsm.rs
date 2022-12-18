

use std::collections::HashMap;

pub trait TransitionEnum {
}

pub trait State {
    fn act(&mut self);
    fn transition_conditions(&self) -> Vec<TransitionOptions>;
    fn set_next(&mut self, transition_index: usize, next: ValidKey);
}

//remover end_execution
#[derive(Debug)]
pub enum TransitionOptions {
    Stay,
    Change(Option<ValidKey>)
}

pub struct FSM {
    states: HashMap<usize, Box<dyn State>>,
    current: usize
}

#[derive(Clone, Copy, Debug)]
pub struct ValidKey {
    k: usize
}

impl FSM {

    fn act(&mut self) {
        self.states
            .entry(self.current)
            .and_modify(|state| state.act());
    }
    fn update_state(&mut self) -> bool {
        self.states
            .get(&self.current)
            .unwrap()
            .transition_conditions()
            .iter()
            .filter_map(|opt|
                {
                    match opt {
                        TransitionOptions::Stay => None,
                        TransitionOptions::Change(next) => {
                            match next {
                                Some(v) => {self.current = v.k; Some(true)},
                                None => Some(false)
                            }
                        },
                    }
                }
            )
            .nth(0)
            .unwrap_or(true)
    }

    pub fn execute(mut self) {
        while self.update_state() {
            self.act();
        }
    }

    pub fn new(initial_state: impl State + 'static) -> (FSM, ValidKey) {
        let map = {
            let mut temp = HashMap::new();
            temp.insert(1 as usize, Box::new(initial_state) as Box<dyn State>);
            temp
        };
        (FSM { states: map, current: 1 }, ValidKey{ k: 1 })
    }

    pub fn add_transition(&mut self, from: ValidKey, transition_index: usize, to: impl State + 'static) -> ValidKey {
        let next_key = self.states.len() + 1;
        self.states.insert(next_key, Box::new(to) as Box <dyn State>);

        self.states
        .entry(from.k)
            .and_modify(|state| 
                state.set_next(transition_index, ValidKey { k: next_key })
            );

        ValidKey { k: next_key }
    }
}
