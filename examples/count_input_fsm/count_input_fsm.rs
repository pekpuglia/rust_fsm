extern crate fsm;
pub(crate) use fsm::*;

mod states;
use states::{counter::*, inputter::*};



fsm_enums!(CountAndInputFSM;
    StartCounter,
    Inputter,
    Counter10,
    Counter20;
    Counter,
    Inputter
);

pub struct CountAndInputFSM {
    start_counter: Counter<CountAndInputFSMStates>,
    inp: Inputter<CountAndInputFSMStates>,
    counter10: Counter<CountAndInputFSMStates>,
    counter20: Counter<CountAndInputFSMStates>,
    current: CountAndInputFSMStateTypes
}

//automatizar new, trait FSM
//automatizar declaração da struct
//fundir com o fsm_enums!

macro_rules! configure_transitions {
    ($state:expr, $($transition:expr => $next:expr),+) => {
        $state
        $(
            .set_next($transition, $next)
        )+
    };
}


impl CountAndInputFSM {
    pub fn new(starting_number: usize) -> CountAndInputFSM {
        let start_counter = configure_transitions!(
            Counter::new(starting_number), CounterTransitions::Zero => CountAndInputFSMStates::Inputter
        );
        

        let inp = configure_transitions!(
            Inputter::new("selecione o próximo estado", "contador 10", "contador 20"),
                InputterTransitions::Transition1 => CountAndInputFSMStates::Counter10,
                InputterTransitions::Transition2 => CountAndInputFSMStates::Counter20
        );
        let counter10 = Counter::new(10);

        let counter20 = Counter::new(20);

        CountAndInputFSM { start_counter: start_counter.clone(), inp, counter10, counter20, current: start_counter.into() }
            
    }
}

impl FSM for CountAndInputFSM {
    type StateTypesEnum = CountAndInputFSMStateTypes;

    type StatesEnum = CountAndInputFSMStates;
    
    fn current_state(&mut self) -> &mut Self::StateTypesEnum {
        &mut self.current
    }

    //automatizar, tirar clones
    fn set_state(&mut self, state: Self::StatesEnum) {
        match state {
            CountAndInputFSMStates::StartCounter => self.current = self.start_counter.clone().into(),
            CountAndInputFSMStates::Inputter => self.current = self.inp.clone().into(),
            CountAndInputFSMStates::Counter10 => self.current = self.counter10.clone().into(),
            CountAndInputFSMStates::Counter20 => self.current = self.counter20.clone().into(),
        }
    }

}
