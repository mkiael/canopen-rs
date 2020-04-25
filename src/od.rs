use std::collections::HashMap;
use std::string::String;

pub enum Object {
    Boolean(bool),
    Integer8(i8),
    Integer16(i16),
    Integer32(i32),
    Unsigned8(u8),
    Unsigned16(u16),
    Unsigned32(u32),
    Real32(f32),
    VisibleString(String),
    OctetString(String),
    UnicodeString(String)
}

pub struct ObjectDictionary {
    pub dict: HashMap<u16, Object>
}

impl ObjectDictionary {
    pub fn new() -> ObjectDictionary {
        ObjectDictionary{ dict: HashMap::new() }
    }
}

#[cfg(test)]
mod tests {
    use crate::od::{ObjectDictionary, Object};

    #[test]
    fn test_insert_and_get() {
        let mut od = ObjectDictionary::new();
        od.dict.insert(0x1000, Object::Unsigned32(0x3));
        match od.dict.get(&0x1000) {
            Some(x) => match x {
                Object::Unsigned32(y) => assert_eq!(0x3, *y),
                _ => assert!(false)
            },
            _ => assert!(false)
        }
    }
}