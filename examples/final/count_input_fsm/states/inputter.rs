use fsm::*;

#[derive(Clone)]
pub struct Inputter<StatesEnum> {
    prompt_text: String,
    answer_1: String,
    answer_2: String,
    chosen: Option<InputterTransitions>,
    map: EnumMap<InputterTransitions, TransitionOptions<StatesEnum>>
}

impl<StatesEnum> Inputter<StatesEnum> {
    pub fn new(prompt_text: & str, answer_1: &'static str, answer_2: &'static str, map: EnumMap<InputterTransitions, TransitionOptions<StatesEnum>>) -> Inputter<StatesEnum> {
        Inputter { prompt_text: prompt_text.to_string(), answer_1: answer_1.to_string(), answer_2: answer_2.to_string(), chosen: None, map }
    }
}


#[derive(Enum, Clone, Copy)]
pub enum InputterTransitions {
    Transition1,
    Transition2
}

impl<SE: Copy> StateBehaviorSuperType<SE> for Inputter<SE> {
    fn act(&mut self, inp: Self::Input) -> Self::Output {
        println!("{}:", self.prompt_text);

        let input =  {
            let mut temp = String::new();
            let ret = std::io::stdin().read_line(&mut temp);
            match ret {
                Ok(_) => Some(temp.trim().to_string()),
                Err(_) => None,
            }
        };

        self.chosen = input.and_then(|string| match string {
            answer if answer == self.answer_1 => {
                println!("selecionado caso {}", self.answer_1);
                Some(InputterTransitions::Transition1)
            },
            answer if answer == self.answer_2 => {
                println!("selecionado caso {}", self.answer_2);    
                Some(InputterTransitions::Transition2)
            },
            answer => {println!("{} é inválido. Casos válidos: {}, {}", answer, self.answer_1, self.answer_2); None}
        });
    }

    fn transition_condition(&self) -> TransitionOptions<SE> {
        StateTransitionsSetup::transition_condition(self, self.map)
    }

    type Input = ();

    type Output = ();
}


impl<StatesEnum: Copy> StateTransitionsSetup<StatesEnum> for Inputter<StatesEnum> {
    

    type TransitionEnum = InputterTransitions;

    fn transition_condition(&self, map: EnumMap<Self::TransitionEnum, TransitionOptions<StatesEnum>>) -> TransitionOptions<StatesEnum> {
        match self.chosen {
            Some(opt) => map[opt],
            None => TransitionOptions::Stay,
        }
    }
}
