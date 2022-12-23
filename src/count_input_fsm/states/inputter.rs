use crate::base_fsm::*;
#[derive(Clone)]
pub struct Inputter<StatesEnum> {
    prompt_text: &'static str,
    answer_1: &'static str,
    next_1: Option<StatesEnum>,
    answer_2: &'static str,
    next_2: Option<StatesEnum>,
    transitions: Vec<TransitionOptions<StatesEnum>>
}

impl<StatesEnum> Inputter<StatesEnum> {
    pub fn new(prompt_text: &'static str, answer_1: &'static str, answer_2: &'static str) -> Inputter<StatesEnum> {
        Inputter { prompt_text, answer_1, next_1: None, answer_2, next_2: None, transitions: vec![TransitionOptions::Stay, TransitionOptions::Stay] }
    }
}

pub enum InputterTransitions {
    Transition1,
    Transition2
}

impl<SE: Copy> StateBehavior<SE> for Inputter<SE> {
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
    fn transition_conditions(&self) -> Vec<TransitionOptions<SE>> {
        self.transitions.as_slice().to_vec()
    }
}

impl<StatesEnum: Copy> State for Inputter<StatesEnum> {
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

