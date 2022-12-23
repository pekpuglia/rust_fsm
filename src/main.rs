mod base_fsm;

use base_fsm::*;

#[derive(Clone)]
struct Counter<SE: Clone> {
    current: usize,
    next: Option<SE>
}

impl<SE: Clone> Counter<SE> {
    fn new(max: usize) -> Counter<SE> {
        Counter { 
            current: max, 
            next: None 
        }
    }
}

enum CounterTransition {
    Zero
}

impl<SE: Copy> State for Counter<SE> {
    
    fn act(&mut self) {
        self.current -= 1;
        println!("{}", self.current);
    }

    fn transition_conditions(&self) -> Vec<TransitionOptions<Self::StatesEnum>> {
        vec![
            match self.current {
                0 => TransitionOptions::Change(self.next),
                _ => TransitionOptions::Stay
            }
        ]
    }

    fn set_next(&mut self, transition_index: Self::TransitionEnum, next: Self::StatesEnum) -> Counter<Self::StatesEnum> {
        self.next = Some(next);
        self.to_owned()
    }

    type TransitionEnum = CounterTransition;

    type StatesEnum = SE;

}

#[derive(Clone)]
struct Inputter<StatesEnum> {
    prompt_text: &'static str,
    answer_1: &'static str,
    next_1: Option<StatesEnum>,
    answer_2: &'static str,
    next_2: Option<StatesEnum>,
    transitions: Vec<TransitionOptions<StatesEnum>>
}

impl<StatesEnum> Inputter<StatesEnum> {
    fn new(prompt_text: &'static str, answer_1: &'static str, answer_2: &'static str) -> Inputter<StatesEnum> {
        Inputter { prompt_text, answer_1, next_1: None, answer_2, next_2: None, transitions: vec![TransitionOptions::Stay, TransitionOptions::Stay] }
    }
}

enum InputterTransitions {
    Transition1,
    Transition2
}

impl<StatesEnum: Copy> State for Inputter<StatesEnum> {
    fn act(&mut self) {
        println!("{}:", self.prompt_text);

        match {
                    let mut temp = String::new();
                    let ret = std::io::stdin().read_line(&mut temp);
                    match ret {
                        Ok(_) => Some(temp.trim().to_string()),
                        Err(_) => None,
                    }
                }
                .and_then(|string| match string {
                    answer if answer == self.answer_1 => {
                        println!("selecionado caso {}", self.answer_1);
                        Some(vec![TransitionOptions::Change(self.next_1), TransitionOptions::Stay])
                    },
                    answer if answer == self.answer_2 => {
                        println!("selecionado caso {}", self.answer_2);    
                        Some(vec![TransitionOptions::Stay, TransitionOptions::Change(self.next_2)])
                    },
                    answer => {println!("{} é inválido. Casos válidos: {}, {}", answer, self.answer_1, self.answer_2); None}
                }) {
            Some(t) => self.transitions = t,
            None => (),
        }

    }

    fn transition_conditions(&self) -> Vec<TransitionOptions<Self::StatesEnum>> {
        self.transitions.as_slice().to_vec()
    }
    //testar: transition_index como &'static usize
    //fazer tipo para vetor de transições com informação sobre o número de transições
    fn set_next(&mut self, transition: Self::TransitionEnum, next: StatesEnum) -> Inputter<StatesEnum> {
        match transition {
            InputterTransitions::Transition1 => self.next_1 = Some(next),
            InputterTransitions::Transition2 => self.next_2 = Some(next),
        };
        self.to_owned()
    }

    type TransitionEnum = InputterTransitions;

    type StatesEnum = StatesEnum;
}


enum FSMTypes<StatesEnum: Clone> {
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


impl<StatesEnum: Copy> StateTypes<StatesEnum> for FSMTypes<StatesEnum> {
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


struct CountAndInputFSM {
    start_counter: Counter<CountAndInputFSMStates>,
    inp: Inputter<CountAndInputFSMStates>,
    counter10: Counter<CountAndInputFSMStates>,
    counter20: Counter<CountAndInputFSMStates>,
    current: FSMTypes<CountAndInputFSMStates>
}

//
#[derive(Clone, Copy)]
enum CountAndInputFSMStates {
    StartCounter,
    Inputter,
    Counter10,
    Counter20
}

impl CountAndInputFSM {
    fn new(starting_number: usize) -> CountAndInputFSM {
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

//usar fsm builder p usar set next generico (não object-safe)?
fn main() {
    let mut fsm = CountAndInputFSM::new(5);
    
    fsm.execute();
}
