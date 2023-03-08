extern crate fsm;
use fsm::*;

mod states;
use states::{counter::*, inputter::*};

//criar internal_new que recebe uma instancia de cada estado e configura
//as transições

FSM!(CountAndInputFSM;
    StartCounter: Counter<CountAndInputFSMStates>,
    Inputter: Inputter<CountAndInputFSMStates>,
    Counter10: Counter<CountAndInputFSMStates>,
    Counter20: Counter<CountAndInputFSMStates>;
    starts with StartCounter;
    StartCounter: [
        CounterTransitions::Zero => CountAndInputFSMStates::Inputter
    ];
    Inputter: [
        InputterTransitions::Transition1 => CountAndInputFSMStates::Counter10,
        InputterTransitions::Transition2 => CountAndInputFSMStates::Counter20
    ]
);

impl CountAndInputFSM {
    pub fn new(starting_number: usize) -> CountAndInputFSM {
        CountAndInputFSM::internal_new(
            Counter::new(starting_number), 
            Inputter::new("selecione o próximo estado", "contador 10", "contador 20"), 
            Counter::new(10), Counter::new(20))
            
    }
}

impl FSM for CountAndInputFSM {
    type StatesEnum = CountAndInputFSMStates;
    
    fn current_state(&mut self) -> &mut dyn StateBehaviorSuperType<Self::StatesEnum> {
        match self.current {
            CountAndInputFSMStates::StartCounter => &mut self.start_counter,
            CountAndInputFSMStates::Inputter => &mut self.inputter,
            CountAndInputFSMStates::Counter10 => &mut self.counter10,
            CountAndInputFSMStates::Counter20 => &mut self.counter20,
        }
    }

    //automatizar, tirar clones
    fn set_state(&mut self, state: Self::StatesEnum) {
        self.current = state;
    }

}
