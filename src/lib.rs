#![no_std]
//permitir que fsm e estados recebam inputs externos?


pub use enum_map::{EnumArray, EnumMap, Enum, enum_map};

pub trait StateBehaviorSuperType<StatesEnum> {
    fn act(&mut self);
    fn transition_condition(&self) -> TransitionOptions<StatesEnum>;
}

//states enum é parâmetro genérico pq 1 estado pode participar de mais de uma fsm
pub trait StateTransitionsSetup<StatesEnum: Copy>: StateBehaviorSuperType<StatesEnum> {
    //associated type porque cada estado só pode ter 1 enum de transições
    type TransitionEnum: EnumArray<TransitionOptions<StatesEnum>>;

    fn transition_condition(&self, map: EnumMap<Self::TransitionEnum, TransitionOptions<StatesEnum>>) -> TransitionOptions<StatesEnum>;
}

#[derive(Clone, Copy, Debug)]
pub enum TransitionOptions<StatesEnum> {
    Stay,
    Change(StatesEnum),
    End
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
            TransitionOptions::Change(next) => {
                self.set_state(next);
                true
            },
            TransitionOptions::End => false
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
        $($states:ident: $types:ty),+
    ) => {
        //StatesEnum
        paste::item!{
            #[derive(Clone, Copy)]
            pub enum [<$fsm_name States>] {
                $(
                    $states
                ),*
            }
        }

        //struct FSM
        paste::item!{
            pub struct $fsm_name {
                $(
                    [<$states:snake>]: $types
                ),* ,
                current: [<$fsm_name States>]
            }
        }

        //impl FSM
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
    };
}