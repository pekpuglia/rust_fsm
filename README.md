# fsm

This crate offers a flexible, `no_std`, finite state machine (FSM) implementation for robotics with a focus on compile-time guarantees and ergonomics.

## Getting Started

To setup a FSM, you need to:

1) Implement the desired states. Each state needs to implement the traits `StateBehaviorSuperType` and `StateTransitionsSetup`. This also requires an enum that specifies the transition conditions for that state (the `TransitionEnum` associated type from the trait `StateTransitionsSetup`). `SE` is a generic parameter that refers to the `StatesEnum` associated type from the FSM, explained later.

2) use the FSM! macro to auto-generate a bunch of boilerplate for the FSM. The syntax for that is

        FSM!($fsm_name:ident;
        $($states:ident: $types:ty),+;
        starts with $first_state:ident;
        $($start_state:ident: [$($transition:expr => $next:expr),*]);*)
        
where you specify the name for the finite state machine, list the states and their respective types (struct style), define the first state for the FSM, and list the transitions for each state between brackets with the syntax `TransitionEnumVariant => NextStateIdentifier`(as defined in the previous sections of the macro). This macro will generate a struct named `fsm_name`, an enum named `fsm_nameStates` containing variants for each state, an implementation of the trait FSM for this struct and a private constructor `internal_new` which takes all states as parameters, configures their transitions and returns the initialized FSM.

3) to use your FSM type, bring the trait `fsm::FSM` into scope, create an FSM instance and use the `execute` method.
