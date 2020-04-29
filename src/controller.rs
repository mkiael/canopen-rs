use crate::message::CanMessage;

#[derive(Default)]
pub struct CanOpenController {
    node_id: u8,
    outgoing_messages: Vec<CanMessage>,
}

impl CanOpenController {
    pub fn new(node_id: u8) -> CanOpenController {
        CanOpenController {
            node_id,
            outgoing_messages: Vec::new(),
        }
    }

    pub fn init(&mut self) {
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
}
