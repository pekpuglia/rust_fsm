// #![no_std]

extern crate fsm;
use fsm::no_clone_obj_safe_fsm::FSM;
mod count_input_fsm;

use count_input_fsm::CountAndInputFSM;

//fazer trait state único
fn main() {
    let mut fsm = CountAndInputFSM::new(5);
    
    fsm.execute();
}
