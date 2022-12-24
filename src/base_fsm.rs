use enum_dispatch::enum_dispatch;
//trait para stateTypesEnum?
#[enum_dispatch]
pub trait StateBehaviorSuperType<StatesEnum> {
    fn act(&mut self);
    fn transition_conditions(&self) -> Vec<TransitionOptions<StatesEnum>>;
}

//states enum é parâmetro genérico pq 1 estado pode participar de mais de uma fsm
pub trait StateTransitionsSetup<StatesEnum> : StateBehaviorSuperType<StatesEnum> {
    //associated type porque cada estado só pode ter 1 enum de transições
    type TransitionEnum;
    
    fn set_next(&mut self, transition: Self::TransitionEnum, next: StatesEnum) -> Self;
}

#[derive(Clone, Copy, Debug)]
pub enum TransitionOptions<StatesEnum> {
    Stay,
    Change(Option<StatesEnum>)
}

pub trait FSM {
    //redundante? - remover na próxima iteração
    type StateTypesEnum: StateBehaviorSuperType<Self::StatesEnum>;

    type StatesEnum: Clone + Copy;

    fn current_state(&mut self) -> &mut Self::StateTypesEnum;

    fn set_state(&mut self, state: Self::StatesEnum);

    fn act(&mut self) {
        self.current_state().act()
    }

    fn update_state(&mut self) -> bool {
        self.current_state()
            .transition_conditions()
            .iter()
            .filter_map(|opt|
                {
                    match opt {
                        TransitionOptions::Stay => None,
                        TransitionOptions::Change(next) => {
                            match next {
                                Some(v) => {self.set_state(*v); Some(true)},
                                None => Some(false)
                            }
                        },
                    }
                }
            )
            .nth(0)
            .unwrap_or(true)
    }

    fn execute(&mut self) {
        while self.update_state() {
            self.act();
        }
    }
}