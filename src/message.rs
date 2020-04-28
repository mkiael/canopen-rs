#[derive(Clone, Copy)]
pub enum Cob {
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

fn get_function_code(cob: Cob) -> u8 {
    match cob {
        Cob::Nmt => 0x0,
        Cob::Sync => 0x1,
        Cob::Time => 0x2,
        Cob::Emcy => 0x1,
        Cob::Pdo1Tx => 0x3,
        Cob::Pdo1Rx => 0x4,
        Cob::Pdo2Tx => 0x5,
        Cob::Pdo2Rx => 0x6,
        Cob::Pdo3Tx => 0x7,
        Cob::Pdo3Rx => 0x8,
        Cob::Pdo4Tx => 0x9,
        Cob::Pdo4Rx => 0xA,
        Cob::SdoTx => 0xB,
        Cob::SdoRx => 0xC,
        Cob::NmtErrorControl => 0xE,
        _ => 0x0,
    }
}

fn is_broadcast_cob(cob: Cob) -> bool {
    match cob {
        Cob::Nmt => true,
        Cob::Sync => true,
        Cob::Time => true,
        _ => false,
    }
}

fn is_p2p_cob(cob: Cob) -> bool {
    match cob {
        Cob::Emcy => true,
        Cob::Pdo1Tx => true,
        Cob::Pdo1Rx => true,
        Cob::Pdo2Tx => true,
        Cob::Pdo2Rx => true,
        Cob::Pdo3Tx => true,
        Cob::Pdo3Rx => true,
        Cob::Pdo4Tx => true,
        Cob::Pdo4Rx => true,
        Cob::SdoTx => true,
        Cob::SdoRx => true,
        Cob::NmtErrorControl => true,
        _ => false,
    }
}

fn get_base_cob_id(cob: Cob) -> u16 {
    let function_code = get_function_code(cob);
    return (function_code as u16) << 7;
}

fn get_broadcast_cob_id(cob: Cob) -> u16 {
    if is_broadcast_cob(cob) {
        return get_base_cob_id(cob);
    } else {
        panic!("Not a broadcast cob");
    }
}

fn get_p2p_cob_id(node_id: u8, cob: Cob) -> u16 {
    if is_p2p_cob(cob) {
        return get_base_cob_id(cob) + (node_id as u16);
    } else {
        panic!("Not a peer to peer cob");
    }
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
    use crate::message::{CanMessage, get_function_code, get_base_cob_id, get_broadcast_cob_id, get_p2p_cob_id, is_broadcast_cob, is_p2p_cob};
    use crate::message::Cob;

    #[test]
    fn test_get_function_code() {
        assert_eq!(get_function_code(Cob::Nmt), 0x0);
        assert_eq!(get_function_code(Cob::Sync), 0x1);
        assert_eq!(get_function_code(Cob::Time), 0x2);
        assert_eq!(get_function_code(Cob::Emcy), 0x1);
        assert_eq!(get_function_code(Cob::Pdo1Tx), 0x3);
        assert_eq!(get_function_code(Cob::Pdo1Rx), 0x4);
        assert_eq!(get_function_code(Cob::Pdo2Tx), 0x5);
        assert_eq!(get_function_code(Cob::Pdo2Rx), 0x6);
        assert_eq!(get_function_code(Cob::Pdo3Tx), 0x7);
        assert_eq!(get_function_code(Cob::Pdo3Rx), 0x8);
        assert_eq!(get_function_code(Cob::Pdo4Tx), 0x9);
        assert_eq!(get_function_code(Cob::Pdo4Rx), 0xA);
        assert_eq!(get_function_code(Cob::SdoTx), 0xB);
        assert_eq!(get_function_code(Cob::SdoRx), 0xC);
        assert_eq!(get_function_code(Cob::NmtErrorControl), 0xE);
        assert_eq!(get_function_code(Cob::Unknown), 0x0);
    }

    #[test]
    fn test_is_broadcast_cob() {
        assert!(is_broadcast_cob(Cob::Nmt));
        assert!(is_broadcast_cob(Cob::Sync));
        assert!(is_broadcast_cob(Cob::Time));
        assert!(!is_broadcast_cob(Cob::Emcy));
        assert!(!is_broadcast_cob(Cob::Pdo1Tx));
        assert!(!is_broadcast_cob(Cob::Pdo1Rx));
        assert!(!is_broadcast_cob(Cob::Pdo2Tx));
        assert!(!is_broadcast_cob(Cob::Pdo2Rx));
        assert!(!is_broadcast_cob(Cob::Pdo3Tx));
        assert!(!is_broadcast_cob(Cob::Pdo3Rx));
        assert!(!is_broadcast_cob(Cob::Pdo4Tx));
        assert!(!is_broadcast_cob(Cob::Pdo4Rx));
        assert!(!is_broadcast_cob(Cob::SdoTx));
        assert!(!is_broadcast_cob(Cob::SdoRx));
        assert!(!is_broadcast_cob(Cob::NmtErrorControl));
    }

    #[test]
    fn test_is_p2p_cob() {
        assert!(!is_p2p_cob(Cob::Nmt));
        assert!(!is_p2p_cob(Cob::Sync));
        assert!(!is_p2p_cob(Cob::Time));
        assert!(is_p2p_cob(Cob::Emcy));
        assert!(is_p2p_cob(Cob::Pdo1Tx));
        assert!(is_p2p_cob(Cob::Pdo1Rx));
        assert!(is_p2p_cob(Cob::Pdo2Tx));
        assert!(is_p2p_cob(Cob::Pdo2Rx));
        assert!(is_p2p_cob(Cob::Pdo3Tx));
        assert!(is_p2p_cob(Cob::Pdo3Rx));
        assert!(is_p2p_cob(Cob::Pdo4Tx));
        assert!(is_p2p_cob(Cob::Pdo4Rx));
        assert!(is_p2p_cob(Cob::SdoTx));
        assert!(is_p2p_cob(Cob::SdoRx));
        assert!(is_p2p_cob(Cob::NmtErrorControl));
    }

    #[test]
    fn test_get_base_cob_id() {
        assert_eq!(get_base_cob_id(Cob::Nmt), 0x0);
        assert_eq!(get_base_cob_id(Cob::Sync), 0x80);
        assert_eq!(get_base_cob_id(Cob::Time), 0x100);
        assert_eq!(get_base_cob_id(Cob::Emcy), 0x80);
        assert_eq!(get_base_cob_id(Cob::Pdo1Tx), 0x180);
        assert_eq!(get_base_cob_id(Cob::Pdo1Rx), 0x200);
        assert_eq!(get_base_cob_id(Cob::Pdo2Tx), 0x280);
        assert_eq!(get_base_cob_id(Cob::Pdo2Rx), 0x300);
        assert_eq!(get_base_cob_id(Cob::Pdo3Tx), 0x380);
        assert_eq!(get_base_cob_id(Cob::Pdo3Rx), 0x400);
        assert_eq!(get_base_cob_id(Cob::Pdo4Tx), 0x480);
        assert_eq!(get_base_cob_id(Cob::Pdo4Rx), 0x500);
        assert_eq!(get_base_cob_id(Cob::SdoTx), 0x580);
        assert_eq!(get_base_cob_id(Cob::SdoRx), 0x600);
        assert_eq!(get_base_cob_id(Cob::NmtErrorControl), 0x700);
        assert_eq!(get_base_cob_id(Cob::Unknown), 0x0);
    }

    #[test]
    fn test_get_broadcast_cob_id() {
        assert_eq!(get_broadcast_cob_id(Cob::Nmt), 0x0);
        assert_eq!(get_broadcast_cob_id(Cob::Sync), 0x80);
        assert_eq!(get_broadcast_cob_id(Cob::Time), 0x100);
    }

    #[test]
    #[should_panic]
    fn test_get_non_broadcast_cob_id() {
        get_broadcast_cob_id(Cob::Emcy);
    }

    #[test]
    fn test_get_p2p_cob_id() {
        assert_eq!(get_p2p_cob_id(0x5, Cob::Emcy), 0x85);
        assert_eq!(get_p2p_cob_id(0xA, Cob::Pdo1Tx), 0x18A);
        assert_eq!(get_p2p_cob_id(0x2, Cob::Pdo1Rx), 0x202);
        assert_eq!(get_p2p_cob_id(0xA, Cob::Pdo2Tx), 0x28A);
        assert_eq!(get_p2p_cob_id(0x2, Cob::Pdo2Rx), 0x302);
        assert_eq!(get_p2p_cob_id(0xA, Cob::Pdo3Tx), 0x38A);
        assert_eq!(get_p2p_cob_id(0x2, Cob::Pdo3Rx), 0x402);
        assert_eq!(get_p2p_cob_id(0xA, Cob::Pdo4Tx), 0x48A);
        assert_eq!(get_p2p_cob_id(0x2, Cob::Pdo4Rx), 0x502);
        assert_eq!(get_p2p_cob_id(0xA, Cob::SdoTx), 0x58A);
        assert_eq!(get_p2p_cob_id(0x2, Cob::SdoRx), 0x602);
        assert_eq!(get_p2p_cob_id(0xA, Cob::NmtErrorControl), 0x70A);
    }

    #[test]
    #[should_panic]
    fn test_get_non_p2p_cob_id() {
        get_p2p_cob_id(0x6, Cob::Sync);
    }

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
