extern crate fsm;
use fsm::*;

mod states;
use states::{counter::*, inputter::*};


FSM!(CountAndInputFSM;
    StartCounter: Counter<CountAndInputFSMStates>,
    Inputter: Inputter<CountAndInputFSMStates>,
    Counter10: Counter<CountAndInputFSMStates>,
    Counter20: Counter<CountAndInputFSMStates>
);

impl CountAndInputFSM {
    pub fn new(starting_number: usize) -> CountAndInputFSM {
        CountAndInputFSM {
            start_counter: Counter::new(starting_number, TransitionOptions::Change(Some(CountAndInputFSMStates::Inputter))),
            inputter: Inputter::new(
                "selecione o próximo estado", 
                "contador 10", 
                "contador 20",
                enum_map! {
                    InputterTransitions::Transition1 => TransitionOptions::Change(Some(CountAndInputFSMStates::Counter10)),
                    InputterTransitions::Transition2 => TransitionOptions::Change(Some(CountAndInputFSMStates::Counter20))
            }),
            counter10: Counter::new(10, TransitionOptions::Change(None)),
            counter20: Counter::new(20, TransitionOptions::Change(None)),
            current: CountAndInputFSMStates::StartCounter,
        }            
    }
}