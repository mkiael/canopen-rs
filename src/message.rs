use crate::cob::{get_broadcast_cob_id, get_p2p_cob_id, Cob};

pub struct CanMessage {
    can_id: u16,
    data: Vec<u8>,
}

impl CanMessage {
    pub fn from_can_id(can_id: u16, data: Vec<u8>) -> CanMessage {
        CanMessage { can_id, data }
    }

    pub fn from_cob(cob: Cob, data: Vec<u8>) -> CanMessage {
        let can_id = get_broadcast_cob_id(cob);
        CanMessage { can_id, data }
    }

    pub fn from_node_id(node_id: u8, cob: Cob, data: Vec<u8>) -> CanMessage {
        let can_id = get_p2p_cob_id(node_id, cob);
        CanMessage { can_id, data }
    }

    pub fn node_id(&self) -> u8 {
        (self.can_id as u8) & 0x7Fu8
    }

    pub fn cob(&self) -> Cob {
        let fc = (self.can_id >> 7) & 0x000Fu16;
        if self.node_id() == 0x0 {
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
        }
    }

    pub fn data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn data_length(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::cob::Cob;
    use crate::message::CanMessage;

    #[test]
    fn test_get_node_id() {
        let msg = CanMessage::from_can_id(0x1B4, Vec::new());
        assert_eq!(msg.node_id(), 0x34);
    }

    #[test]
    fn test_get_nmt_cob() {
        let msg = CanMessage::from_can_id(0x0, Vec::new());
        assert_eq!(msg.cob(), Cob::Nmt);
    }

    #[test]
    fn test_get_sync_cob() {
        let msg = CanMessage::from_can_id(0x80, Vec::new());
        assert_eq!(msg.cob(), Cob::Sync);
    }

    #[test]
    fn test_get_time_cob() {
        let msg = CanMessage::from_can_id(0x100, Vec::new());
        assert_eq!(msg.cob(), Cob::Time);
    }

    #[test]
    fn test_get_unknown_broadcast_cob() {
        let msg = CanMessage::from_can_id(0x780, Vec::new());
        assert_eq!(msg.cob(), Cob::Unknown);
    }

    #[test]
    fn test_get_emcy_cob() {
        let msg = CanMessage::from_can_id(0xAD, Vec::new());
        assert_eq!(msg.cob(), Cob::Emcy);
    }

    #[test]
    fn test_get_pdo1tx_cob() {
        let msg = CanMessage::from_can_id(0x1AD, Vec::new());
        assert_eq!(msg.cob(), Cob::Pdo1Tx);
    }

    #[test]
    fn test_get_pdo1rx_cob() {
        let msg = CanMessage::from_can_id(0x22D, Vec::new());
        assert_eq!(msg.cob(), Cob::Pdo1Rx);
    }

    #[test]
    fn test_get_pdo2tx_cob() {
        let msg = CanMessage::from_can_id(0x2AD, Vec::new());
        assert_eq!(msg.cob(), Cob::Pdo2Tx);
    }

    #[test]
    fn test_get_pdo2rx_cob() {
        let msg = CanMessage::from_can_id(0x32D, Vec::new());
        assert_eq!(msg.cob(), Cob::Pdo2Rx);
    }

    #[test]
    fn test_get_pdo3tx_cob() {
        let msg = CanMessage::from_can_id(0x3AD, Vec::new());
        assert_eq!(msg.cob(), Cob::Pdo3Tx);
    }

    #[test]
    fn test_get_pdo3rx_cob() {
        let msg = CanMessage::from_can_id(0x42D, Vec::new());
        assert_eq!(msg.cob(), Cob::Pdo3Rx);
    }

    #[test]
    fn test_get_pdo4tx_cob() {
        let msg = CanMessage::from_can_id(0x4AD, Vec::new());
        assert_eq!(msg.cob(), Cob::Pdo4Tx);
    }

    #[test]
    fn test_get_pdo4rx_cob() {
        let msg = CanMessage::from_can_id(0x52D, Vec::new());
        assert_eq!(msg.cob(), Cob::Pdo4Rx);
    }

    #[test]
    fn test_get_sdotx_cob() {
        let msg = CanMessage::from_can_id(0x5AD, Vec::new());
        assert_eq!(msg.cob(), Cob::SdoTx);
    }

    #[test]
    fn test_get_sdorx_cob() {
        let msg = CanMessage::from_can_id(0x62D, Vec::new());
        assert_eq!(msg.cob(), Cob::SdoRx);
    }

    #[test]
    fn test_get_nmt_error_control_cob() {
        let msg = CanMessage::from_can_id(0x72D, Vec::new());
        assert_eq!(msg.cob(), Cob::NmtErrorControl);
    }

    #[test]
    fn test_get_unknown_peer_to_peer_cob() {
        let msg = CanMessage::from_can_id(0x7AD, Vec::new());
        assert_eq!(msg.cob(), Cob::Unknown);
    }

    #[test]
    fn test_get_data() {
        let data = vec![0x1, 0x3, 0x3, 0x7];
        let msg = CanMessage::from_can_id(0x1B4, data);
        assert_eq!(*msg.data(), vec![0x1, 0x3, 0x3, 0x7]);
    }

    #[test]
    fn test_get_data_length() {
        let data = vec![0x1, 0x3, 0x3, 0x7];
        let msg = CanMessage::from_can_id(0x1B4, data);
        assert_eq!(msg.data_length(), 4);
    }

    #[test]
    fn test_from_cob() {
        let msg = CanMessage::from_cob(Cob::Sync, vec![0x0]);
        assert_eq!(msg.can_id, 0x80);
    }

    #[test]
    fn test_from_node_id() {
        let msg = CanMessage::from_node_id(0x12, Cob::Pdo1Tx, vec![0x0, 0x1, 0x2]);
        assert_eq!(msg.can_id, 0x192);
    }
}
