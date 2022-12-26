use fsm::*;
//usar strings - checar estado com dados na heap
#[derive(Clone)]
pub struct Inputter<StatesEnum> {
    prompt_text: String,
    answer_1: String,
    next_1: Option<StatesEnum>,
    answer_2: String,
    next_2: Option<StatesEnum>,
    transitions: Vec<TransitionOptions<StatesEnum>>
}

impl<StatesEnum> Inputter<StatesEnum> {
    pub fn new(prompt_text: & str, answer_1: &'static str, answer_2: &'static str) -> Inputter<StatesEnum> {
        Inputter { prompt_text: prompt_text.to_string(), answer_1: answer_1.to_string(), next_1: None, answer_2: answer_2.to_string(), next_2: None, transitions: vec![TransitionOptions::Stay, TransitionOptions::Stay] }
    }
}


#[derive(EnumCount)]
pub enum InputterTransitions {
    Transition1,
    Transition2
}

impl<SE: Copy> StateBehaviorSuperType<SE> for Inputter<SE> {
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

    fn transition_condition(&self) -> TransitionOptions<SE> {
        self.transition_condition_impl()
    }
}

const INPUTTER_TRANSITION_COUNT: usize = 2;

impl<StatesEnum: Copy> StateTransitionsSetup<StatesEnum, INPUTTER_TRANSITION_COUNT> for Inputter<StatesEnum> {
    fn set_next(&mut self, transition: Self::TransitionEnum, next: StatesEnum) -> Inputter<StatesEnum> {
        match transition {
            InputterTransitions::Transition1 => self.next_1 = Some(next),
            InputterTransitions::Transition2 => self.next_2 = Some(next),
        };
        self.to_owned()
    }

    fn transition_conditions(&self) -> heapless::Vec<TransitionOptions<StatesEnum>, INPUTTER_TRANSITION_COUNT> {
       heapless::Vec::from_slice(&self.transitions).expect("transitions deve ter INPUTTER_TRANSITION_COUNT elementos")
    }

    type TransitionEnum = InputterTransitions;
}

generate_assertion!(Inputter);