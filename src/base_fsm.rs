#![no_std]
use enum_dispatch::enum_dispatch;
//trait para stateTypesEnum?
#[enum_dispatch]
pub trait StateBehaviorSuperType<StatesEnum> {
    fn act(&mut self);
    fn ref_transition_conditions(&self) -> &[TransitionOptions<StatesEnum>];
}

pub trait TransitionAssertedState {}

macro_rules! generate_assertion {
    ($state:ident) => {
        const fn assert() {
            static_assertions::const_assert_eq!(<paste::paste!([<$state Transitions>])>::COUNT, paste::paste!{[<$state:upper _TRANSITION_COUNT>]});
        }

        impl<SE: Clone> TransitionAssertedState for $state<SE> {}
    };
}
pub(crate) use generate_assertion;
pub(crate) use strum::EnumCount;
pub(crate) use strum_macros::EnumCount;
//states enum é parâmetro genérico pq 1 estado pode participar de mais de uma fsm
pub trait StateTransitionsSetup<StatesEnum, const NUMBER_OF_TRANSITIONS: usize> : StateBehaviorSuperType<StatesEnum> + TransitionAssertedState {
    //associated type porque cada estado só pode ter 1 enum de transições
    type TransitionEnum: EnumCount;
    fn transition_conditions(&self) -> heapless::Vec<TransitionOptions<StatesEnum>, NUMBER_OF_TRANSITIONS>;

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
        let ret = self.current_state()
            .ref_transition_conditions()
            .iter()
            .filter_map(|opt|
                {
                    match opt {
                        TransitionOptions::Stay => None,
                        TransitionOptions::Change(next) => {
                            match next {
                                Some(v) => {Some((Some(*v), true))},
                                None => Some((None, false))
                            }
                        },
                    }
                }
            )
            .nth(0)
            .unwrap_or((None,true));
            ret.0.and_then(|state| Some(self.set_state(state)));
            ret.1
    }

    fn execute(&mut self) {
        while self.update_state() {
            self.act();
        }
    }
}