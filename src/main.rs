mod base_fsm;

use base_fsm::*;

mod count_input_fsm;

use count_input_fsm::CountAndInputFSM;

fn main() {
    let mut fsm = CountAndInputFSM::new(5);
    
    fsm.execute();
}
