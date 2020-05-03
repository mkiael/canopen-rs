use crate::cob::Cob;
use crate::message::CanMessage;
use crate::service::node_control::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NmtState {
    Initialising,
    PreOperational,
    Operational,
    Stopped,
}

pub struct CanOpenController {
    node_id: u8,
    nmt_state: NmtState,
    outgoing_messages: Vec<CanMessage>,
}

impl CanOpenController {
    pub fn new(node_id: u8) -> CanOpenController {
        CanOpenController {
            node_id,
            nmt_state: NmtState::Initialising,
            outgoing_messages: Vec::new(),
        }
    }

    pub fn init(&mut self) {
        // TODO: Setup object dictionary

        self.reset_node();
    }

    pub fn process(&mut self, can_message: CanMessage) {
        if let Cob::Nmt = can_message.cob() {
            match handle_nmt_message(self.node_id, can_message) {
                NodeCommand::StartNode => self.set_nmt_state(NmtState::Operational),
                NodeCommand::StopNode => self.set_nmt_state(NmtState::Stopped),
                NodeCommand::EnterPreOperational => self.set_nmt_state(NmtState::PreOperational),
                NodeCommand::ResetNode => self.reset_node(),
                NodeCommand::ResetCommunication => self.reset_communication(),
                _ => {}
            }
        }
    }

    pub fn update(&mut self, _dt: std::time::Duration) {}

    pub fn fetch(&mut self) -> Vec<CanMessage> {
        let mut messages = Vec::new();
        for msg_it in self.outgoing_messages.drain(..) {
            messages.push(msg_it);
        }
        messages
    }

    pub fn nmt_state(&self) -> NmtState {
        self.nmt_state
    }

    fn set_nmt_state(&mut self, nmt_state: NmtState) {
        self.nmt_state = nmt_state;
    }

    fn reset_node(&mut self) {
        self.set_nmt_state(NmtState::Initialising);

        // TODO: Reset application parameters

        self.reset_communication();
    }

    fn reset_communication(&mut self) {
        self.set_nmt_state(NmtState::Initialising);

        // TODO: Reset communication parameters

        self.send_boot_up();
        self.set_nmt_state(NmtState::PreOperational);
    }

    fn send_boot_up(&mut self) {
        self.outgoing_messages.push(CanMessage::from_node_id(
            self.node_id,
            Cob::NmtErrorControl,
            vec![0x0],
        ));
    }
}
