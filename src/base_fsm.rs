use enum_dispatch::enum_dispatch;
//trait para stateTypesEnum?
#[enum_dispatch]
pub trait StateBehaviorSuperType<StatesEnum> {
    fn act(&mut self);
    fn transition_condition(&self) -> TransitionOptions<StatesEnum>;
}

pub trait TransitionAssertedState {}

macro_rules! generate_assertion {
    ($state:ident) => {
        const fn assert() {
            static_assertions::const_assert_eq!(<paste::paste!([<$state Transitions>])>::COUNT, paste::paste!{[<$state:upper _TRANSITION_COUNT>]});
        }

        impl<SE: Copy> TransitionAssertedState for $state<SE> {}
    };
}
pub(crate) use generate_assertion;
pub(crate) use strum::EnumCount;
pub(crate) use strum_macros::EnumCount;
//states enum é parâmetro genérico pq 1 estado pode participar de mais de uma fsm
pub trait StateTransitionsSetup<StatesEnum: Copy, const NUMBER_OF_TRANSITIONS: usize> : StateBehaviorSuperType<StatesEnum> + TransitionAssertedState {
    //associated type porque cada estado só pode ter 1 enum de transições
    type TransitionEnum: EnumCount;
    fn transition_conditions(&self) -> heapless::Vec<TransitionOptions<StatesEnum>, NUMBER_OF_TRANSITIONS>;

    fn transition_condition(&self) -> TransitionOptions<StatesEnum> {
        self.transition_conditions()
            .iter()
            .filter_map(|opt|
                {
                    match opt {
                        TransitionOptions::Stay => None,
                        TransitionOptions::Change(_) => Some(*opt),
                    }
                }
            )
            .nth(0)
            .unwrap_or(TransitionOptions::Stay)
    }

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
        match self.current_state().transition_condition() {
            TransitionOptions::Stay => true,
            TransitionOptions::Change(next) => match next {
                Some(state) => {self.set_state(state); true},
                None => false
            },
        }
    }

    fn execute(&mut self) {
        while self.update_state() {
            self.act();
        }
    }
}