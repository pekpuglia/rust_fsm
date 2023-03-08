#![no_std]
use strum::IntoEnumIterator;
pub mod clone_not_obj_safe_fsm;

//implementar para o states enum - remover requisito de object safety e fazer trait único
pub trait StateBehaviorSuperType<StatesEnum> {
    fn act(&mut self);
    fn transition_condition(&self) -> TransitionOptions<StatesEnum>;
}

pub trait TransitionEnumTrait<StatesEnum: Copy> : IntoEnumIterator {
    type State;
    fn transition_conditions(&self, state: &Self::State) -> TransitionOptions<StatesEnum>;
}

//states enum é parâmetro genérico pq 1 estado pode participar de mais de uma fsm
pub trait StateTransitionsSetup<StatesEnum: Copy> {
    //associated type porque cada estado só pode ter 1 enum de transições
    type TransitionEnum: TransitionEnumTrait<StatesEnum, State = Self>;

    fn set_next(&mut self, transition: Self::TransitionEnum, next: StatesEnum) -> Self;

    fn transition_condition(&self) -> TransitionOptions<StatesEnum> {
        Self::TransitionEnum::iter()
            .map(|variant| variant.transition_conditions(self))
            .filter_map(|opt|
                {
                    match opt {
                        TransitionOptions::Stay => None,
                        TransitionOptions::Change(_) => Some(opt),
                    }
                }
            )
            .nth(0)
            .unwrap_or(TransitionOptions::Stay)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum TransitionOptions<StatesEnum> {
    Stay,
    Change(Option<StatesEnum>)
}

pub trait FSM {

    type StatesEnum: Clone + Copy;

    fn current_state(&mut self) -> &mut dyn StateBehaviorSuperType<Self::StatesEnum>;

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

    fn execute(mut self) where Self: Sized {
        while self.update_state() {
            self.act();
        }
    }
}

#[macro_export]
macro_rules! FSM {
    (
        $fsm_name:ident; 
        $($states:ident: $types:ty),+;
        starts with $first_state:ident;
        $($start_state:ident: [$($transition:expr => $next:expr),*]);*
    ) => {
        paste::item!{
            #[derive(Clone, Copy)]
            pub enum [<$fsm_name States>] {
                $(
                    $states
                ),*
            }
        }

        paste::item!{
            pub struct $fsm_name {
                $(
                    [<$states:snake>]: $types
                ),* ,
                current: [<$fsm_name States>]
            }
        }

        paste::item!{
            impl FSM for $fsm_name {
                type StatesEnum = [<$fsm_name States>];

                fn current_state(&mut self) -> &mut dyn StateBehaviorSuperType<Self::StatesEnum> {
                    match self.current {
                        $(
                            [<$fsm_name States>]::$states => &mut self.[<$states:snake>]
                        ),+
                    }
                }

                    fn set_state(&mut self, state: Self::StatesEnum) {
                        self.current = state;
                    }
            }
        }

        paste::item!{
            impl $fsm_name {
                fn internal_new($(mut [<$states:snake>]: $types),+) -> $fsm_name {
                    $(
                        let [<$start_state:snake>] =
                        [<$start_state:snake>]
                        $(
                            .set_next($transition, $next)
                        )*
                    );*;

                    $fsm_name {
                        $(
                            [<$states:snake>]
                        ),+,
                        current: [<$fsm_name States>]::$first_state
                    }
                }
            }
        }
    };
}