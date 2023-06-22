// // #![no_std]

extern crate fsm;
use fsm::FSM;
mod count_input_fsm;

use count_input_fsm::CountAndInputFSM;

fn main() {
    let mut fsm = CountAndInputFSM::new(5);
    
    while fsm.update_state() {
        fsm.act(())
    }
}
