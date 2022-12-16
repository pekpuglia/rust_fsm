mod base_fsm;
use base_fsm::*;

#[derive(Clone)]
struct Counter {
    current: usize,
    next: Option<ValidKey>
}

impl Counter {
    fn new(max: usize) -> Counter {
        Counter { 
            current: max, 
            next: None 
        }
    }
}

impl State for Counter {
    fn act(&mut self) {
        self.current -= 1;
        println!("{}", self.current);
    }

    fn transition_conditions(&self) -> Vec<TransitionOptions> {
        vec![
            match self.current {
                0 => TransitionOptions::Change(self.next),
                _ => TransitionOptions::Stay
            }
        ]
    }

    fn set_next(&mut self, transition_index: usize, next: ValidKey) {
        self.next = Some(next)
    }
}

fn main() {
    let (mut fsm, v1) = base_fsm::FSM::new(Counter::new(10));

    
    
    fsm.execute();
}
