use crate::cob::Cob;
use crate::message::CanMessage;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NmtState {
    Initialising,
    PreOperational,
    //Operational,
    //Stopped
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
        // TODO: Perform proper initialisation
        self.nmt_state = NmtState::PreOperational;

        // Send boot-up message
        self.outgoing_messages.push(CanMessage::from_node_id(
            self.node_id,
            Cob::NmtErrorControl,
            vec![0x0],
        ));
    }

    pub fn process(&mut self, _can_message: CanMessage) {}

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
}
