use crate::base_fsm::*;

mod states;
use states::{counter::*, inputter::*};
pub enum FSMTypes<StatesEnum: Clone> {
    Counter(Counter<StatesEnum>),
    Inputter(Inputter<StatesEnum>)
}

impl<StatesEnum: Clone> From<Counter<StatesEnum>> for FSMTypes<StatesEnum> {
    fn from(c: Counter<StatesEnum>) -> Self {
        FSMTypes::Counter(c)
    }
}

impl<StatesEnum: Clone> From<Inputter<StatesEnum>> for FSMTypes<StatesEnum> {
    fn from(i: Inputter<StatesEnum>) -> Self {
        FSMTypes::Inputter(i)
    }
}


impl<StatesEnum: Copy> StateBehavior<StatesEnum> for FSMTypes<StatesEnum> {
    fn act(&mut self) {
        match self {
            FSMTypes::Counter(state) => state.act(),
            FSMTypes::Inputter(state) => state.act(),
        }
    }

    fn transition_conditions(&self) -> Vec<TransitionOptions<StatesEnum>> {
        match self {
            FSMTypes::Counter(state) => state.transition_conditions(),
            FSMTypes::Inputter(state) => state.transition_conditions(),
        }
    }
}


pub struct CountAndInputFSM {
    start_counter: Counter<CountAndInputFSMStates>,
    inp: Inputter<CountAndInputFSMStates>,
    counter10: Counter<CountAndInputFSMStates>,
    counter20: Counter<CountAndInputFSMStates>,
    current: FSMTypes<CountAndInputFSMStates>
}

//
#[derive(Clone, Copy)]
pub enum CountAndInputFSMStates {
    StartCounter,
    Inputter,
    Counter10,
    Counter20
}

impl CountAndInputFSM {
    pub fn new(starting_number: usize) -> CountAndInputFSM {
        let start_counter = Counter::new(starting_number)
            .set_next(CounterTransition::Zero, CountAndInputFSMStates::Inputter);

        let inp = Inputter::new("selecione o próximo estado", "contador 10", "contador 20")
            .set_next(InputterTransitions::Transition1, CountAndInputFSMStates::Counter10)
            .set_next(InputterTransitions::Transition2, CountAndInputFSMStates::Counter20);

        let counter10 = Counter::new(10);

        let counter20 = Counter::new(20);

        CountAndInputFSM { start_counter: start_counter.clone().to_owned(), inp: inp.to_owned(), counter10, counter20, current: start_counter.into() }
            
    }
}

impl FSM for CountAndInputFSM {
    type StateTypesEnum = FSMTypes<Self::StatesEnum>;

    type StatesEnum = CountAndInputFSMStates;
    
    fn current_state(&mut self) -> &mut Self::StateTypesEnum {
        &mut self.current
    }

    fn set_state(&mut self, state: Self::StatesEnum) {
        match state {
            CountAndInputFSMStates::StartCounter => self.current = self.start_counter.clone().into(),
            CountAndInputFSMStates::Inputter => self.current = self.inp.clone().into(),
            CountAndInputFSMStates::Counter10 => self.current = self.counter10.clone().into(),
            CountAndInputFSMStates::Counter20 => self.current = self.counter20.clone().into(),
        }
    }

}