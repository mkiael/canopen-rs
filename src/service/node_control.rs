use crate::cob::Cob;
use crate::controller::NmtState;
use crate::message::CanMessage;

#[derive(PartialEq, Debug)]
pub enum NodeCommand {
    ChangeNmtState(NmtState),
    None,
}

pub struct NodeControl {
    node_id: u8,
}

impl NodeControl {
    pub fn new(node_id: u8) -> NodeControl {
        NodeControl { node_id }
    }

    pub fn process(&self, can_message: CanMessage) -> NodeCommand {
        if self.is_message_valid(&can_message) {
            match can_message.data()[0] {
                0x1 => NodeCommand::ChangeNmtState(NmtState::Operational),
                _ => NodeCommand::None,
            }
        } else {
            NodeCommand::None
        }
    }

    fn is_message_valid(&self, can_message: &CanMessage) -> bool {
        (can_message.cob() == Cob::Nmt)
            && (can_message.data_length() == 2)
            && (can_message.data()[1] == self.node_id)
    }
}

#[cfg(test)]
mod tests {
    use crate::cob::Cob;
    use crate::controller::NmtState;
    use crate::message::CanMessage;
    use crate::service::node_control::NodeCommand;
    use crate::service::node_control::NodeControl;

    #[test]
    fn test_start_remote_node() {
        let node_control = NodeControl::new(0x4);
        let cmd = node_control.process(CanMessage::from_cob(Cob::Nmt, vec![0x1, 0x4]));
        if let NodeCommand::ChangeNmtState(nmt_state) = cmd {
            assert_eq!(nmt_state, NmtState::Operational);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn test_start_remote_node_wrong_node_id() {
        let node_control = NodeControl::new(0x4);
        let cmd = node_control.process(CanMessage::from_cob(Cob::Nmt, vec![0x1, 0x5]));
        assert_eq!(cmd, NodeCommand::None);
    }
}
