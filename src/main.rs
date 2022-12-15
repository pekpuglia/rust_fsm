use std::collections::HashMap;

//trait com add_transitions?
trait State {
    fn act(&mut self);
    fn transition_conditions(&self) -> Vec<TransitionOptions>;
}

enum TransitionOptions {
    Stay,
    Change(usize),
    EndExecution
}


struct FSM {
    states: HashMap<usize, Box<dyn State>>,
    current: usize
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
}




fn main() {
    println!("Hello, world!");
}
