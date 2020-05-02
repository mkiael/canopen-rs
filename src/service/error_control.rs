use crate::od::ObjectDictionary;

pub struct ErrorControl {
    node_id: u8,
}

impl ErrorControl {
    pub fn new(node_id: u8) -> Self {
        ErrorControl { node_id }
    }
}