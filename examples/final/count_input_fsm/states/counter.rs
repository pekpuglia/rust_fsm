use fsm::*;
#[derive(Clone)]
pub struct Counter<SE: Clone> {
    current: usize,
    map: EnumMap<CounterTransitions, TransitionOptions<SE>>
}

impl<SE: Copy> Counter<SE> {
    pub fn new(max: usize, next: TransitionOptions<SE>) -> Counter<SE> {
        Counter { 
            current: max,
            map: enum_map! {
                CounterTransitions::Zero => next
            }, 
            
        }
    }
}

#[derive(Enum)]
pub enum CounterTransitions {
    Zero
}

impl<SE: Copy> StateBehaviorSuperType<SE> for Counter<SE> {
    fn act(&mut self) {
        self.current -= 1;
        println!("{}", self.current);
    }

    fn transition_condition(&self) -> TransitionOptions<SE> {
        StateTransitionsSetup::transition_condition(self, self.map)
    }
}


impl<SE: Copy> StateTransitionsSetup<SE> for Counter<SE> {
    
    type TransitionEnum = CounterTransitions;

    fn transition_condition(&self, map: EnumMap<Self::TransitionEnum, TransitionOptions<SE>>) -> TransitionOptions<SE> {
        match self.current {
            0 => map[CounterTransitions::Zero],
            _ => TransitionOptions::Stay
        }
    }


}
