mod base_fsm;
use base_fsm::*;

#[derive(Clone)]
struct Counter {
    current: usize,
    exit_condition: fn(&Counter) -> TransitionOptions
}

impl Counter {
    fn new(max: usize) -> Counter {
        Counter { current: max, exit_condition: (|c:&Counter| TransitionOptions::Stay)}
    }
}

impl State for Counter {
    fn act(&mut self) {
        self.current -= 1;
    }

    fn transition_conditions(&self) -> Vec<TransitionOptions> {
        vec![(self.exit_condition)(self)]
    }
}

impl StateWithTransitions for Counter {
    fn add_transitions(&mut self, condition: fn(&Self) -> TransitionOptions) {
        self.exit_condition = condition;
    }
}


fn main() {
    let mut c1 = Counter::new(10);
    c1.add_transitions(|c| match c.current == 0 {
        true => TransitionOptions::Change(2),
        false => TransitionOptions::Stay,
    });
    //retornar ref mutavel p estado adicionado (p ser possível adicionar transições)
    let (mut fsm, v1) = base_fsm::FSM::new(c1);
    fsm.insert_state(Counter::new(20));
}
