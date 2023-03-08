use strum::IntoEnumIterator;

pub trait TransitionEnumTrait<StatesEnum: Copy> : IntoEnumIterator {
    type State;
    fn transition_conditions(&self, state: &Self::State) -> TransitionOptions<StatesEnum>;
}

//states enum é parâmetro genérico pq 1 estado pode participar de mais de uma fsm
pub trait State<StatesEnum: Copy> {
    //associated type porque cada estado só pode ter 1 enum de transições
    type TransitionEnum: TransitionEnumTrait<StatesEnum, State = Self>;

    fn act(&mut self);

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

pub trait StateTypesTrait<StatesEnum> {
    fn act(&mut self);
    fn transition_condition(&self) -> TransitionOptions<StatesEnum>;
}

pub trait FSM {

    type StatesEnum: Clone + Copy;

    type StatesTypesEnum: StateTypesTrait<Self::StatesEnum>;

    fn current_state(&mut self) -> &mut Self::StatesTypesEnum;

    fn set_state(&mut self, state: Self::StatesEnum);

    fn act(&mut self) {
        self.current_state().act();
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
macro_rules! clone_not_obejct_safe_fsm {
    ($fsm_name:ident; $($states:ident),+) => {
        paste::item!{
            #[derive(Clone, Copy)]
            pub enum [<$fsm_name States>] {
                $(
                    $states
                ),*
            }
        }

        
    };
}