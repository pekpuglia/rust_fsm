// #![no_std]
#[forbid(clippy::todo)]
mod base_fsm;

use base_fsm::*;

mod count_input_fsm;

use count_input_fsm::CountAndInputFSM;

//transformar em lib
//organizar código
//merge com a main?
fn main() {
    let mut fsm = CountAndInputFSM::new(5);
    
    fsm.execute();
}
