extern crate canopen_rs;

use canopen_rs::controller::CanOpenController;
use canopen_rs::message::CanMessage;

#[test]
fn test_can_open_controller_basic_usage() {
    let mut controller = CanOpenController::new();
    controller.init();
    controller.process(CanMessage::from_can_id(0x0, vec![]));
    let msgs = controller.fetch();

    assert!(msgs.is_empty());
}
