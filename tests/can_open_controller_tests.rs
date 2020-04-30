extern crate canopen_rs;

use canopen_rs::cob::Cob;
use canopen_rs::controller::{CanOpenController, NmtState};
use canopen_rs::message::CanMessage;

fn is_boot_up_message(can_message: Option<CanMessage>, node_id: u8) -> bool {
    match can_message {
        Some(msg) => {
            (msg.node_id() == node_id)
                && (msg.cob() == Cob::NmtErrorControl)
                && (*msg.data() == vec![0x0])
        }
        None => false,
    }
}

#[test]
fn test_can_open_controller_init() {
    let mut controller = CanOpenController::new(0x1A);

    controller.init();
    let mut msgs = controller.fetch();

    assert!(is_boot_up_message(msgs.pop(), 0x1A));
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

#[test]
fn test_can_open_controller_stopped() {
    let mut controller = CanOpenController::new(0x1A);

    controller.init();
    controller.process(CanMessage::from_cob(Cob::Nmt, vec![0x2, 0x1A]));

    assert_eq!(controller.nmt_state(), NmtState::Stopped);
}

#[test]
fn test_can_open_controller_enter_pre_operational() {
    let mut controller = CanOpenController::new(0x1A);

    controller.init();

    controller.process(CanMessage::from_cob(Cob::Nmt, vec![0x1, 0x1A]));
    controller.process(CanMessage::from_cob(Cob::Nmt, vec![0x80, 0x1A]));

    assert_eq!(controller.nmt_state(), NmtState::PreOperational);
}

#[test]
fn test_can_open_controller_reset_node() {
    let mut controller = CanOpenController::new(0x1A);

    controller.init();
    controller.process(CanMessage::from_cob(Cob::Nmt, vec![0x81, 0x1A]));

    assert!(is_boot_up_message(controller.fetch().pop(), 0x1A));
    assert_eq!(controller.nmt_state(), NmtState::PreOperational);
}

#[test]
fn test_can_open_controller_reset_communication() {
    let mut controller = CanOpenController::new(0x1A);

    controller.init();
    controller.process(CanMessage::from_cob(Cob::Nmt, vec![0x82, 0x1A]));

    assert!(is_boot_up_message(controller.fetch().pop(), 0x1A));
    assert_eq!(controller.nmt_state(), NmtState::PreOperational);
}
