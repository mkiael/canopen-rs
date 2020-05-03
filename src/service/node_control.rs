use crate::cob::Cob;
use crate::message::CanMessage;

#[derive(PartialEq, Debug)]
pub enum NodeCommand {
    StartNode,
    StopNode,
    EnterPreOperational,
    ResetNode,
    ResetCommunication,
    None,
}

pub fn handle_nmt_message(node_id: u8, can_message: CanMessage) -> NodeCommand {
    if is_message_valid(node_id, &can_message) {
        match can_message.data()[0] {
            0x1 => NodeCommand::StartNode,
            0x2 => NodeCommand::StopNode,
            0x80 => NodeCommand::EnterPreOperational,
            0x81 => NodeCommand::ResetNode,
            0x82 => NodeCommand::ResetCommunication,
            _ => NodeCommand::None,
        }
    } else {
        NodeCommand::None
    }
}

fn is_message_valid(node_id: u8, can_message: &CanMessage) -> bool {
    (can_message.cob() == Cob::Nmt)
        && (can_message.data_length() == 2)
        && (can_message.data()[1] == node_id)
}

#[cfg(test)]
mod tests {
    use crate::cob::Cob;
    use crate::message::CanMessage;
    use crate::service::node_control::*;

    fn test_node_command(node_cmd: NodeCommand, cs: u8) {
        let cmd = handle_nmt_message(0x4, CanMessage::from_cob(Cob::Nmt, vec![cs, 0x4]));
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
    fn test_enter_pre_operational() {
        test_node_command(NodeCommand::EnterPreOperational, 0x80);
    }

    #[test]
    fn test_reset_node() {
        test_node_command(NodeCommand::ResetNode, 0x81);
    }

    #[test]
    fn test_reset_communication() {
        test_node_command(NodeCommand::ResetCommunication, 0x82);
    }

    #[test]
    fn test_invalid_message_not_nmt() {
        let cmd = handle_nmt_message(0x4, CanMessage::from_cob(Cob::Sync, vec![]));
        assert_eq!(cmd, NodeCommand::None);
    }

    #[test]
    fn test_invalid_message_not_correct_data_length() {
        let cmd = handle_nmt_message(0x4, CanMessage::from_cob(Cob::Nmt, vec![]));
        assert_eq!(cmd, NodeCommand::None);
    }

    #[test]
    fn test_invalid_message_not_same_node_id() {
        let cmd = handle_nmt_message(0x4, CanMessage::from_cob(Cob::Nmt, vec![0x1, 0x5]));
        assert_eq!(cmd, NodeCommand::None);
    }

    #[test]
    fn test_invalid_message_unknown_command_specifier() {
        let cmd = handle_nmt_message(0x4, CanMessage::from_cob(Cob::Nmt, vec![0x63, 0x4]));
        assert_eq!(cmd, NodeCommand::None);
    }
}
