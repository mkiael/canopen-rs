use crate::cob::Cob;

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

    pub fn cob(&self) -> Cob {
        let fc = (self.can_id >> 7) & 0x000Fu16;
        return if self.node_id() == 0x0 {
            match fc {
                0x0 => Cob::Nmt,
                0x1 => Cob::Sync,
                0x2 => Cob::Time,
                _ => Cob::Unknown,
            }
        } else {
            match fc {
                0x1 => Cob::Emcy,
                0x3 => Cob::Pdo1Tx,
                0x4 => Cob::Pdo1Rx,
                0x5 => Cob::Pdo2Tx,
                0x6 => Cob::Pdo2Rx,
                0x7 => Cob::Pdo3Tx,
                0x8 => Cob::Pdo3Rx,
                0x9 => Cob::Pdo4Tx,
                0xA => Cob::Pdo4Rx,
                0xB => Cob::SdoTx,
                0xC => Cob::SdoRx,
                0xE => Cob::NmtErrorControl,
                _ => Cob::Unknown,
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
    use crate::cob::Cob;

    #[test]
    fn test_get_node_id() {
        let msg = CanMessage::new(0x1B4, Vec::new());
        assert_eq!(msg.node_id(), 0x34);
    }

    #[test]
    fn test_get_nmt_cob() {
        let msg = CanMessage::new(0x0, Vec::new());
        assert!(match msg.cob() {
            Cob::Nmt => true,
            _ => false,
        });
    }

    #[test]
    fn test_get_sync_cob() {
        let msg = CanMessage::new(0x80, Vec::new());
        assert!(match msg.cob() {
            Cob::Sync => true,
            _ => false,
        });
    }

    #[test]
    fn test_get_time_cob() {
        let msg = CanMessage::new(0x100, Vec::new());
        assert!(match msg.cob() {
            Cob::Time => true,
            _ => false,
        });
    }

    #[test]
    fn test_get_unknown_broadcast_cob() {
        let msg = CanMessage::new(0x780, Vec::new());
        assert!(match msg.cob() {
            Cob::Unknown => true,
            _ => false,
        });
    }

    #[test]
    fn test_get_emcy_cob() {
        let msg = CanMessage::new(0xAD, Vec::new());
        assert!(match msg.cob() {
            Cob::Emcy => true,
            _ => false,
        });
    }

    #[test]
    fn test_get_pdo1tx_cob() {
        let msg = CanMessage::new(0x1AD, Vec::new());
        assert!(match msg.cob() {
            Cob::Pdo1Tx => true,
            _ => false,
        });
    }

    #[test]
    fn test_get_pdo1rx_cob() {
        let msg = CanMessage::new(0x22D, Vec::new());
        assert!(match msg.cob() {
            Cob::Pdo1Rx => true,
            _ => false,
        });
    }

    #[test]
    fn test_get_pdo2tx_cob() {
        let msg = CanMessage::new(0x2AD, Vec::new());
        assert!(match msg.cob() {
            Cob::Pdo2Tx => true,
            _ => false,
        });
    }

    #[test]
    fn test_get_pdo2rx_cob() {
        let msg = CanMessage::new(0x32D, Vec::new());
        assert!(match msg.cob() {
            Cob::Pdo2Rx => true,
            _ => false,
        });
    }

    #[test]
    fn test_get_pdo3tx_cob() {
        let msg = CanMessage::new(0x3AD, Vec::new());
        assert!(match msg.cob() {
            Cob::Pdo3Tx => true,
            _ => false,
        });
    }

    #[test]
    fn test_get_pdo3rx_cob() {
        let msg = CanMessage::new(0x42D, Vec::new());
        assert!(match msg.cob() {
            Cob::Pdo3Rx => true,
            _ => false,
        });
    }

    #[test]
    fn test_get_pdo4tx_cob() {
        let msg = CanMessage::new(0x4AD, Vec::new());
        assert!(match msg.cob() {
            Cob::Pdo4Tx => true,
            _ => false,
        });
    }

    #[test]
    fn test_get_pdo4rx_cob() {
        let msg = CanMessage::new(0x52D, Vec::new());
        assert!(match msg.cob() {
            Cob::Pdo4Rx => true,
            _ => false,
        });
    }

    #[test]
    fn test_get_sdotx_cob() {
        let msg = CanMessage::new(0x5AD, Vec::new());
        assert!(match msg.cob() {
            Cob::SdoTx => true,
            _ => false,
        });
    }

    #[test]
    fn test_get_sdorx_cob() {
        let msg = CanMessage::new(0x62D, Vec::new());
        assert!(match msg.cob() {
            Cob::SdoRx => true,
            _ => false,
        });
    }

    #[test]
    fn test_get_nmt_error_control_cob() {
        let msg = CanMessage::new(0x72D, Vec::new());
        assert!(match msg.cob() {
            Cob::NmtErrorControl => true,
            _ => false,
        });
    }

    #[test]
    fn test_get_unknown_peer_to_peer_cob() {
        let msg = CanMessage::new(0x7AD, Vec::new());
        assert!(match msg.cob() {
            Cob::Unknown => true,
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
