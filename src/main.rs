mod base_fsm;
use base_fsm::*;

#[derive(Clone)]
struct Counter {
    current: usize,
    next: Option<ValidKey>
}

impl Counter {
    fn new(max: usize) -> Counter {
        Counter { 
            current: max, 
            next: None 
        }
    }
}

impl State for Counter {
    
    fn act(&mut self) {
        self.current -= 1;
        println!("{}", self.current);
    }

    fn transition_conditions(&self) -> Vec<TransitionOptions> {
        vec![
            match self.current {
                0 => TransitionOptions::Change(self.next),
                _ => TransitionOptions::Stay
            }
        ]
    }

    fn set_next(&mut self, transition_index: usize, next: ValidKey) {
        self.next = Some(next)
    }

}

struct Inputter {
    prompt_text: &'static str,
    answer_1: &'static str,
    next_1: Option<ValidKey>,
    answer_2: &'static str,
    next_2: Option<ValidKey>,
    transitions: Vec<TransitionOptions>
}

impl Inputter {
    fn new(prompt_text: &'static str, answer_1: &'static str, answer_2: &'static str) -> Inputter {
        Inputter { prompt_text, answer_1, next_1: None, answer_2, next_2: None, transitions: vec![TransitionOptions::Stay, TransitionOptions::Stay] }
    }
}

impl State for Inputter {
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

    fn transition_conditions(&self) -> Vec<TransitionOptions> {
        self.transitions.as_slice().to_vec()
    }
    //testar: transition_index como &'static usize
    //fazer tipo para vetor de transições com informação sobre o número de transições
    fn set_next(&mut self, transition_index: usize, next: ValidKey) {
        match transition_index {
            1 => self.next_1 = Some(next),
            2 => self.next_2 = Some(next),
            n => println!("o índice {n} é inválido. Todas as transições são Stay. Índices válidos: 1, 2")
        }
    }
}

fn main() {
    let (mut fsm, v1) = base_fsm::FSM::new(Counter::new(5));

    let v2 = fsm.add_transition(v1, 1, Inputter::new("selecione o próximo estado", "contador 10", "contador 20"));

    let v3 = fsm.add_transition(v2, 1, Counter::new(10));

    let v4 = fsm.add_transition(v2, 2, Counter::new(20));
    
    fsm.execute();
}
