
use crate::base_fsm::*;
#[derive(Clone)]
pub struct Counter<SE: Clone> {
    current: usize,
    next: Option<SE>,
}

impl<SE: Clone> Counter<SE> {
    pub fn new(max: usize) -> Counter<SE> {
        Counter { 
            current: max, 
            next: None,
        }
    }
}



#[derive(EnumCount)]
pub enum CounterTransitions {
    Zero
}

impl<SE: Copy> StateBehaviorSuperType<SE> for Counter<SE> {
    fn act(&mut self) {
        self.current -= 1;
        println!("{}", self.current);
    }

    fn transition_condition(&self) -> TransitionOptions<SE> {
        StateTransitionsSetup::transition_condition(self)
    }
}

const COUNTER_TRANSITION_COUNT: usize = 1;

impl<SE: Copy> StateTransitionsSetup<SE, COUNTER_TRANSITION_COUNT> for Counter<SE> {
    
    type TransitionEnum = CounterTransitions;

    fn set_next(&mut self, _transition: Self::TransitionEnum, next: SE) -> Counter<SE> {
        self.next = Some(next);
        self.to_owned()
    }

    fn transition_conditions(&self) -> heapless::Vec<TransitionOptions<SE>, COUNTER_TRANSITION_COUNT> {
        heapless::Vec::from_slice(&[match self.current {
            0 => TransitionOptions::Change(self.next),
            _ => TransitionOptions::Stay
        }]).expect("o valor de retorno deve ter COUNTER_TRANSITION_COUNT elementos")
    }

}

generate_assertion!(Counter);