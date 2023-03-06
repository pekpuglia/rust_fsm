# fsm

This crate offers a flexible, `no_std`, finite state machine (FSM) implementation for robotics with a focus on compile-time guarantees and ergonomics.

## Getting Started

To setup a FSM, you need to:

1) Implement the desired states. Each state needs to implement the traits `StateBehaviorSuperType` and `StateTransitionsSetup`. This also requires an enum that specifies the transition conditions for that state (the `TransitionEnum` associated type from the trait `StateTransitionsSetup`). `SE` is a generic parameter that refers to the `StatesEnum` associated type from the FSM, explained later.

2) todo!()
