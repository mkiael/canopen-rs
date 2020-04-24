extern crate canopen_rs;

use canopen_rs::canopen::CanMessage;
use canopen_rs::canopen::CanOpenController;

#[test]
fn test_can_open_controller_basic_usage() {
    let mut controller = CanOpenController::new();
    controller.init();
    controller.process(CanMessage::new(0x0, vec![]));
    let msgs = controller.fetch();

    assert!(msgs.is_empty());
}

