extern crate fsm;
use fsm::{clone_not_obj_safe_fsm::*, clone_not_obejct_safe_fsm};

mod states;
use states::{counter::*, inputter::*};



clone_not_obejct_safe_fsm!(CountAndInputFSM;
    StartCounter,
    Inputter,
    Counter10,
    Counter20
);

pub enum CountAndInputTypes {
    StartCounter(Counter<CountAndInputFSMStates>),
    Inputter(Inputter<CountAndInputFSMStates>),
    Counter10(Counter<CountAndInputFSMStates>),
    Counter20(Counter<CountAndInputFSMStates>)
}

impl StateTypesTrait<CountAndInputFSMStates> for CountAndInputTypes {
    fn act(&mut self) {
        match self {
            CountAndInputTypes::StartCounter(sc) => sc.act(),
            CountAndInputTypes::Inputter(i) => i.act(),
            CountAndInputTypes::Counter10(c10) => c10.act(),
            CountAndInputTypes::Counter20(c20) => c20.act(),
        }
    }

    fn transition_condition(&self) -> TransitionOptions<CountAndInputFSMStates> {
        match self {
            CountAndInputTypes::StartCounter(sc) => sc.transition_condition(),
            CountAndInputTypes::Inputter(i) => i.transition_condition(),
            CountAndInputTypes::Counter10(c10) => c10.transition_condition(),
            CountAndInputTypes::Counter20(c20) => c20.transition_condition(),
        }
    }
}
pub struct CountAndInputFSM {
    start_counter: Counter<CountAndInputFSMStates>,
    inp:           Inputter<CountAndInputFSMStates>,
    counter10:     Counter<CountAndInputFSMStates>,
    counter20:     Counter<CountAndInputFSMStates>,
    current:       CountAndInputTypes
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

        CountAndInputFSM { start_counter: start_counter.clone(), inp, counter10, counter20, current: CountAndInputTypes::StartCounter(start_counter) }
            
    }
}

impl FSM for CountAndInputFSM {
    type StatesEnum = CountAndInputFSMStates;
    
    fn current_state(&mut self) -> &mut Self::StatesTypesEnum {
        &mut self.current
    }

    fn set_state(&mut self, state: Self::StatesEnum) {
        self.current = match state {
            CountAndInputFSMStates::StartCounter => CountAndInputTypes::StartCounter(self.start_counter.clone()),
            CountAndInputFSMStates::Inputter => CountAndInputTypes::Inputter(self.inp.clone()),
            CountAndInputFSMStates::Counter10 => CountAndInputTypes::Counter10(self.counter10.clone()),
            CountAndInputFSMStates::Counter20 => CountAndInputTypes::Counter20(self.counter20.clone()),
        }
    }

    type StatesTypesEnum = CountAndInputTypes;

}
