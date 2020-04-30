use crate::cob::Cob;
use crate::message::CanMessage;
use crate::service::node_control::NodeCommand;
use crate::service::node_control::NodeControl;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum NmtState {
    Initialising,
    PreOperational,
    Operational,
    Stopped
}

pub struct CanOpenController {
    node_id: u8,
    nmt_state: NmtState,
    node_control: NodeControl,
    outgoing_messages: Vec<CanMessage>,
}

impl CanOpenController {
    pub fn new(node_id: u8) -> CanOpenController {
        CanOpenController {
            node_id,
            node_control: NodeControl::new(node_id),
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

    pub fn process(&mut self, can_message: CanMessage) {
        if let Cob::Nmt = can_message.cob() {
            self.execute_node_control(can_message)
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

    fn execute_node_control(&mut self, can_message: CanMessage) {
        match self.node_control.process(can_message) {
            NodeCommand::StartNode => self.set_nmt_state(NmtState::Operational),
            NodeCommand::StopNode => self.set_nmt_state(NmtState::Stopped),
            NodeCommand::EnterPreOperational => self.set_nmt_state(NmtState::PreOperational),
            _ => {}
        }
    }

    fn set_nmt_state(&mut self, nmt_state: NmtState) {
        self.nmt_state = nmt_state;
    }
}
