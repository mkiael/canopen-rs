pub mod canopen {

    pub struct CanMessage {
        can_id: u16,
        data: Vec<u8>
    }

    impl CanMessage {
        pub fn new(can_id: u16, data: Vec<u8>) -> CanMessage {
            CanMessage{
                can_id,
                data
            }
        }

        pub fn node_id(&self) -> u8 {
            (self.can_id as u8) & 0x7fu8
        }

        pub fn data_length(&self) -> usize {
            self.data.len()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::canopen::CanMessage;

    #[test]
    fn get_node_id() {
        let msg = CanMessage::new(0x1B4, Vec::new());
        assert_eq!(msg.node_id(), 0x34);
    }

    #[test]
    fn get_data_length() {
        let data = vec![0x1, 0x3, 0x3, 0x7];
        let msg = CanMessage::new(0x1B4, data);
        assert_eq!(msg.data_length(), 4);
    }
}

