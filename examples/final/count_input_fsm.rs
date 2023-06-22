extern crate fsm;
use fsm::*;

mod states;
use states::{counter::*, inputter::*};


FSM!(CountAndInputFSM: () => String;
    StartCounter: Counter<CountAndInputFSMStates>,
    Inputter: Inputter<CountAndInputFSMStates>,
    Counter10: Counter<CountAndInputFSMStates>,
    Counter20: Counter<CountAndInputFSMStates>
);

impl CountAndInputFSM {
    pub fn new(starting_number: usize) -> CountAndInputFSM {
        CountAndInputFSM {
            start_counter: Counter::new(starting_number, TransitionOptions::Change(CountAndInputFSMStates::Inputter)),
            inputter: Inputter::new(
                "selecione o próximo estado", 
                "contador 10", 
                "contador 20",
                enum_map! {
                    InputterTransitions::Transition1 => TransitionOptions::Change(CountAndInputFSMStates::Counter10),
                    InputterTransitions::Transition2 => TransitionOptions::Change(CountAndInputFSMStates::Counter20)
            }),
            counter10: Counter::new(10, TransitionOptions::End),
            counter20: Counter::new(20, TransitionOptions::End),
            current: CountAndInputFSMStates::StartCounter,
        }            
    }
}