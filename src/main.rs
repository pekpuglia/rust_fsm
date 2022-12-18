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

enum CounterTransitions {
    Zero
}

impl TransitionEnum for CounterTransitions {
    
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
//usar https://stackoverflow.com/questions/28136739/is-it-possible-to-control-the-size-of-an-array-using-the-type-parameter-of-a-gen
//const generics p indicar numero de transicoes do estado
//OU TRAIT PARA TRANSITION ENUM - corrigir construtores - não funciona (associated type c/ box)
//fazer fsm ser apenas trait
fn main() {
    let (mut fsm, v1) = base_fsm::FSM::new(Counter::new(10));

    let v2 = fsm.add_transition(v1, 1, Counter::new(20));
    
    fsm.execute();
}
