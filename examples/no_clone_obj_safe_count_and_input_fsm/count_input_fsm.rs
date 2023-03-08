extern crate fsm;
use fsm::{no_clone_obj_safe_fsm::*, NCOSfsm};

mod states;
use states::{counter::*, inputter::*};

//criar internal_new que recebe uma instancia de cada estado e configura
//as transições

NCOSfsm!(CountAndInputFSM;
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
        

        let inputter = configure_transitions!(
            Inputter::new("selecione o próximo estado", "contador 10", "contador 20"),
                InputterTransitions::Transition1 => CountAndInputFSMStates::Counter10,
                InputterTransitions::Transition2 => CountAndInputFSMStates::Counter20
        );
        let counter10 = Counter::new(10);

        let counter20 = Counter::new(20);

        CountAndInputFSM { start_counter: start_counter, inputter, counter10, counter20, current: CountAndInputFSMStates::StartCounter }
            
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
