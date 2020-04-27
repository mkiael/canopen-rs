pub enum FunctionCode {
    Nmt,
    Sync,
    Time,
    Emcy,
    Pdo1Tx,
    Pdo1Rx,
    Pdo2Tx,
    Pdo2Rx,
    Pdo3Tx,
    Pdo3Rx,
    Pdo4Tx,
    Pdo4Rx,
    SdoTx,
    SdoRx,
    NmtErrorControl,
    Unknown,
}

pub struct CanMessage {
    can_id: u16,
    data: Vec<u8>,
}

impl CanMessage {
    pub fn new(can_id: u16, data: Vec<u8>) -> CanMessage {
        CanMessage { can_id, data }
    }

    pub fn node_id(&self) -> u8 {
        (self.can_id as u8) & 0x7Fu8
    }

    pub fn function_code(&self) -> FunctionCode {
        let fc = (self.can_id >> 7) & 0x000Fu16;
        return if self.node_id() == 0x0 {
            match fc {
                0x0 => FunctionCode::Nmt,
                0x1 => FunctionCode::Sync,
                0x2 => FunctionCode::Time,
                _ => FunctionCode::Unknown,
            }
        } else {
            match fc {
                0x1 => FunctionCode::Emcy,
                0x3 => FunctionCode::Pdo1Tx,
                0x4 => FunctionCode::Pdo1Rx,
                0x5 => FunctionCode::Pdo2Tx,
                0x6 => FunctionCode::Pdo2Rx,
                0x7 => FunctionCode::Pdo3Tx,
                0x8 => FunctionCode::Pdo3Rx,
                0x9 => FunctionCode::Pdo4Tx,
                0xA => FunctionCode::Pdo4Rx,
                0xB => FunctionCode::SdoTx,
                0xC => FunctionCode::SdoRx,
                0xE => FunctionCode::NmtErrorControl,
                _ => FunctionCode::Unknown,
            }
        };
    }

    pub fn data_length(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::message::CanMessage;
    use crate::message::FunctionCode;

    #[test]
    fn test_get_node_id() {
        let msg = CanMessage::new(0x1B4, Vec::new());
        assert_eq!(msg.node_id(), 0x34);
    }

    #[test]
    fn test_get_nmt_function_code() {
        let msg = CanMessage::new(0x0, Vec::new());
        assert!(match msg.function_code() {
            FunctionCode::Nmt => true,
            _ => false,
        });
    }

    #[test]
    fn test_get_sync_function_code() {
        let msg = CanMessage::new(0x80, Vec::new());
        assert!(match msg.function_code() {
            FunctionCode::Sync => true,
            _ => false,
        });
    }

    #[test]
    fn test_get_time_function_code() {
        let msg = CanMessage::new(0x100, Vec::new());
        assert!(match msg.function_code() {
            FunctionCode::Time => true,
            _ => false,
        });
    }

    #[test]
    fn test_get_unknown_broadcast_function_code() {
        let msg = CanMessage::new(0x780, Vec::new());
        assert!(match msg.function_code() {
            FunctionCode::Unknown => true,
            _ => false,
        });
    }

    #[test]
    fn test_get_emcy_function_code() {
        let msg = CanMessage::new(0xAD, Vec::new());
        assert!(match msg.function_code() {
            FunctionCode::Emcy => true,
            _ => false,
        });
    }

    #[test]
    fn test_get_pdo1tx_function_code() {
        let msg = CanMessage::new(0x1AD, Vec::new());
        assert!(match msg.function_code() {
            FunctionCode::Pdo1Tx => true,
            _ => false,
        });
    }

    #[test]
    fn test_get_pdo1rx_function_code() {
        let msg = CanMessage::new(0x22D, Vec::new());
        assert!(match msg.function_code() {
            FunctionCode::Pdo1Rx => true,
            _ => false,
        });
    }

    #[test]
    fn test_get_pdo2tx_function_code() {
        let msg = CanMessage::new(0x2AD, Vec::new());
        assert!(match msg.function_code() {
            FunctionCode::Pdo2Tx => true,
            _ => false,
        });
    }

    #[test]
    fn test_get_pdo2rx_function_code() {
        let msg = CanMessage::new(0x32D, Vec::new());
        assert!(match msg.function_code() {
            FunctionCode::Pdo2Rx => true,
            _ => false,
        });
    }

    #[test]
    fn test_get_pdo3tx_function_code() {
        let msg = CanMessage::new(0x3AD, Vec::new());
        assert!(match msg.function_code() {
            FunctionCode::Pdo3Tx => true,
            _ => false,
        });
    }

    #[test]
    fn test_get_pdo3rx_function_code() {
        let msg = CanMessage::new(0x42D, Vec::new());
        assert!(match msg.function_code() {
            FunctionCode::Pdo3Rx => true,
            _ => false,
        });
    }

    #[test]
    fn test_get_pdo4tx_function_code() {
        let msg = CanMessage::new(0x4AD, Vec::new());
        assert!(match msg.function_code() {
            FunctionCode::Pdo4Tx => true,
            _ => false,
        });
    }

    #[test]
    fn test_get_pdo4rx_function_code() {
        let msg = CanMessage::new(0x52D, Vec::new());
        assert!(match msg.function_code() {
            FunctionCode::Pdo4Rx => true,
            _ => false,
        });
    }

    #[test]
    fn test_get_sdotx_function_code() {
        let msg = CanMessage::new(0x5AD, Vec::new());
        assert!(match msg.function_code() {
            FunctionCode::SdoTx => true,
            _ => false,
        });
    }

    #[test]
    fn test_get_sdorx_function_code() {
        let msg = CanMessage::new(0x62D, Vec::new());
        assert!(match msg.function_code() {
            FunctionCode::SdoRx => true,
            _ => false,
        });
    }

    #[test]
    fn test_get_nmt_error_control_function_code() {
        let msg = CanMessage::new(0x72D, Vec::new());
        assert!(match msg.function_code() {
            FunctionCode::NmtErrorControl => true,
            _ => false,
        });
    }

    #[test]
    fn test_get_unknown_peer_to_peer_function_code() {
        let msg = CanMessage::new(0x7AD, Vec::new());
        assert!(match msg.function_code() {
            FunctionCode::Unknown => true,
            _ => false,
        });
    }

    #[test]
    fn test_get_data_length() {
        let data = vec![0x1, 0x3, 0x3, 0x7];
        let msg = CanMessage::new(0x1B4, data);
        assert_eq!(msg.data_length(), 4);
    }
}
