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
    fn transition_condition(&self) -> TransitionOptions<SE> {
        StateTransitionsSetup::transition_condition(self, self.map)
    }

    type Input = ();

    type Output = String;

    fn act(&mut self, inp: Self::Input) -> Self::Output {
        self.current -= 1;
        self.current.to_string()
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
