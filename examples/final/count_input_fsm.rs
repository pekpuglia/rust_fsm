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
            Counter::new(starting_number, TransitionOptions::Change(Some(CountAndInputFSMStates::Inputter))), 
            Inputter::new(
                "selecione o próximo estado", 
                "contador 10", 
                "contador 20",
            enum_map! {
                InputterTransitions::Transition1 => TransitionOptions::Change(Some(CountAndInputFSMStates::Counter10)),
                InputterTransitions::Transition2 => TransitionOptions::Change(Some(CountAndInputFSMStates::Counter20))
            }), 
            Counter::new(10, TransitionOptions::Change(None)), Counter::new(20, TransitionOptions::Change(None)))
            
    }
}