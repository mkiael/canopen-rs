use std::collections::HashMap;
use std::string::String;

#[derive(Hash, Eq, PartialEq)]
pub struct ObjectKey {
    index: u16,
    sub_index: u8,
}

pub enum ObjectValue {
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
    dict: HashMap<ObjectKey, ObjectValue>
}

impl ObjectDictionary {
    pub fn new() -> ObjectDictionary {
        ObjectDictionary{ dict: HashMap::new() }
    }

    pub fn add(&mut self, index: u16, sub_index: u8, value: ObjectValue) {
        self.dict.insert(ObjectKey{index, sub_index}, value);
    }

    pub fn write(&mut self, index: u16, sub_index: u8, value: ObjectValue) {
        if let Some(x) = self.dict.get_mut(&ObjectKey { index, sub_index }) {
            if std::mem::discriminant(x) == std::mem::discriminant(&value) {
                *x = value;
            }
        }
    }

    pub fn read(&self, index: u16, sub_index: u8) -> Option<&ObjectValue> {
        return self.dict.get(&ObjectKey{index, sub_index });
    }
}