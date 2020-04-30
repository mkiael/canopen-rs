extern crate canopen_rs;

use canopen_rs::cob::Cob;
use canopen_rs::controller::{CanOpenController, NmtState};
use canopen_rs::message::CanMessage;

#[test]
fn test_can_open_controller_init() {
    let mut controller = CanOpenController::new(0x1A);

    controller.init();
    let mut msgs = controller.fetch();

    if let Some(msg) = msgs.pop() {
        assert_eq!(msg.node_id(), 0x1A);
        assert_eq!(msg.cob(), Cob::NmtErrorControl);
        assert_eq!(*msg.data(), vec![0x0]);
    } else {
        assert!(false);
    }
}

#[test]
fn test_can_open_controller_nmt_state() {
    let mut controller = CanOpenController::new(0x1A);

    assert_eq!(controller.nmt_state(), NmtState::Initialising);

    controller.init();

    assert_eq!(controller.nmt_state(), NmtState::PreOperational);
}

#[test]
fn test_can_open_controller_consume_messages() {
    let mut controller = CanOpenController::new(0x1A);

    controller.init();

    let first_msgs = controller.fetch();

    assert!(!first_msgs.is_empty());

    let second_msgs = controller.fetch();

    assert!(second_msgs.is_empty());
}

#[test]
fn test_can_open_controller_enter_operational() {
    let mut controller = CanOpenController::new(0x1A);

    controller.init();
    controller.process(CanMessage::from_cob(Cob::Nmt, vec![0x1, 0x1A]));

    assert_eq!(controller.nmt_state(), NmtState::Operational);
}
