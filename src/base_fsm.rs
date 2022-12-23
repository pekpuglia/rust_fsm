
pub trait State {
    type TransitionEnum;

    type StatesEnum : Clone;

    fn act(&mut self);
    fn transition_conditions(&self) -> Vec<TransitionOptions<Self::StatesEnum>>;
    //melhorar o set_next
    fn set_next(&mut self, transition: Self::TransitionEnum, next: Self::StatesEnum) -> Self;
}

#[derive(Clone, Copy, Debug)]
pub enum TransitionOptions<StatesEnum> {
    Stay,
    Change(Option<StatesEnum>)
}

//deve ser supertrait do State p/ usar com enum dispatch
pub trait StateTypes<StatesEnum> {
    fn act(&mut self);
    fn transition_conditions(&self) -> Vec<TransitionOptions<StatesEnum>>;

}

pub trait FSM {
    //redundante?
    type StateTypesEnum: StateTypes<Self::StatesEnum>;

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