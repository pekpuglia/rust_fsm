# rust_fsm

This crate offers a flexible, `no_std`, finite state machine (FSM) implementation for robotics with a focus on compile-time guarantees and ergonomics.

## Getting Started

To setup a FSM, you need to:

1) Implement the desired states. Each state needs to implement the traits `StateBehaviorSuperType` and `StateTransitionsSetup`. This also requires an enum that specifies the transition conditions for that state (the `TransitionEnum` associated type from the trait `StateTransitionsSetup`). You may wish to create a field of type `EnumMap` to specify the destination states after each transition.

2) use the FSM! macro to auto-generate a bunch of boilerplate for the FSM. The syntax for that is

        FSM!($fsm_name:ident;
        $($states:ident: $types:ty),+;
  
    where you specify the name for the finite state machine, list the states and their respective types (struct style). This macro will generate a struct named `fsm_name`, an enum named `fsm_nameStates` containing variants for each state, an implementation of the trait FSM for this struct. You need to create a constructor where you specify the initial state for the machine (`current` field for the generated struct).

3) to use your FSM type, bring the trait `fsm::FSM` into scope, create an FSM instance and use the `execute` method.

If you have any trouble, the `examples/final` directory might be of help.
