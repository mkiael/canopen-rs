use crate::cob::Cob;
use crate::message::CanMessage;

#[derive(PartialEq, Debug)]
pub enum NodeCommand {
    StartNode,
    StopNode,
    //EnterPreOperational,
    //ResetNode,
    //ResetCommunication,
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
                0x1 => NodeCommand::StartNode,
                0x2 => NodeCommand::StopNode,
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
    use crate::message::CanMessage;
    use crate::service::node_control::NodeCommand;
    use crate::service::node_control::NodeControl;

    fn test_node_command(node_cmd: NodeCommand, cs: u8) {
        let node_control = NodeControl::new(0x4);
        let cmd = node_control.process(CanMessage::from_cob(Cob::Nmt, vec![cs, 0x4]));
        assert_eq!(cmd, node_cmd);
    }

    #[test]
    fn test_start_remote_node() {
        test_node_command(NodeCommand::StartNode, 0x1);
    }

    #[test]
    fn test_stop_remote_node() {
        test_node_command(NodeCommand::StopNode, 0x2);
    }

    #[test]
    fn test_invalid_message_not_nmt() {
        let node_control = NodeControl::new(0x4);
        let cmd = node_control.process(CanMessage::from_cob(Cob::Sync, vec![]));
        assert_eq!(cmd, NodeCommand::None);
    }

    #[test]
    fn test_invalid_message_not_correct_data_length() {
        let node_control = NodeControl::new(0x4);
        let cmd = node_control.process(CanMessage::from_cob(Cob::Nmt, vec![]));
        assert_eq!(cmd, NodeCommand::None);
    }

    #[test]
    fn test_invalid_message_not_same_node_id() {
        let node_control = NodeControl::new(0x4);
        let cmd = node_control.process(CanMessage::from_cob(Cob::Nmt, vec![0x1, 0x5]));
        assert_eq!(cmd, NodeCommand::None);
    }

    #[test]
    fn test_invalid_message_unknown_command_specifier() {
        let node_control = NodeControl::new(0x4);
        let cmd = node_control.process(CanMessage::from_cob(Cob::Nmt, vec![0x63, 0x4]));
        assert_eq!(cmd, NodeCommand::None);
    }
}
