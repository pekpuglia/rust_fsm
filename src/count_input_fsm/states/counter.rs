use crate::base_fsm::*;
#[derive(Clone)]
pub struct Counter<SE: Clone> {
    current: usize,
    next: Option<SE>
}

impl<SE: Clone> Counter<SE> {
    pub fn new(max: usize) -> Counter<SE> {
        Counter { 
            current: max, 
            next: None 
        }
    }
}

pub enum CounterTransition {
    Zero
}

impl<SE: Copy> StateBehavior<SE> for Counter<SE> {
    fn act(&mut self) {
        self.current -= 1;
        println!("{}", self.current);
    }

    fn transition_conditions(&self) -> Vec<TransitionOptions<SE>> {
        vec![
            match self.current {
                0 => TransitionOptions::Change(self.next),
                _ => TransitionOptions::Stay
            }
        ]
    }
}

impl<SE: Copy> State for Counter<SE> {
    
    type TransitionEnum = CounterTransition;

    type StatesEnum = SE;

    fn set_next(&mut self, transition_index: Self::TransitionEnum, next: Self::StatesEnum) -> Counter<Self::StatesEnum> {
        self.next = Some(next);
        self.to_owned()
    }
}