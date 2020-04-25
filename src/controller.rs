use crate::message::CanMessage;

pub struct CanOpenController {
    pub outgoing_messages: Vec<CanMessage>
}

impl CanOpenController {
    pub fn new() -> CanOpenController {
        CanOpenController{ outgoing_messages: Vec::new() }
    }

    pub fn init(&mut self) {
    }

    pub fn process(&mut self, _can_message: CanMessage) {
    }

    pub fn update(&mut self, _dt: std::time::Duration) {
    }

    pub fn fetch(&mut self) -> Vec<CanMessage> {
        let mut messages = Vec::new();
        for msg_it in self.outgoing_messages.drain(..) {
            messages.push(msg_it);
        }
        return  messages;
    }
}