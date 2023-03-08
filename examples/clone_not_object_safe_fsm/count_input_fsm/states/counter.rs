use fsm::clone_not_obj_safe_fsm::*;
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
//iterar sobre enum, fazer trait p/ transition enum?
use strum_macros::EnumIter;
#[derive(EnumIter)]
pub enum CounterTransitions {
    Zero
}

impl<StatesEnum:Copy> TransitionEnumTrait<StatesEnum> for CounterTransitions {
    type State = Counter<StatesEnum>;

    fn transition_conditions(&self, state: &Self::State) -> TransitionOptions<StatesEnum> {
        match self {
            CounterTransitions::Zero => match state.current {
                0 => TransitionOptions::Change(state.next),
                _ => TransitionOptions::Stay
            },
        }
    }
}

impl<SE: Copy> State<SE> for Counter<SE> {
    fn act(&mut self) {
        self.current -= 1;
        println!("{}", self.current);
    }
    
    type TransitionEnum = CounterTransitions;
    
    fn set_next(&mut self, _transition: Self::TransitionEnum, next: SE) -> Counter<SE> {
        self.next = Some(next);
        self.to_owned()
    }
}