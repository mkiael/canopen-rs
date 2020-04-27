use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::string::String;

#[derive(Hash, Eq, PartialEq)]
struct ObjectKey {
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
    UnicodeString(String),
}

pub trait ObjectSubscriber {
    fn object_updated(&mut self, index: &u16, sub_index: &u8, value: &ObjectValue);
}

struct Object {
    index: u16,
    sub_index: u8,
    value: ObjectValue,
    subscribers: Vec<Rc<RefCell<dyn ObjectSubscriber>>>,
}

impl Object {
    pub fn write(&mut self, value: ObjectValue) {
        if std::mem::discriminant(&self.value) == std::mem::discriminant(&value) {
            self.value = value;
            for subscriber in self.subscribers.iter_mut() {
                subscriber.borrow_mut().object_updated(&self.index, &self.sub_index, &self.value);
            }
        }
    }

    pub fn read(&self) -> &ObjectValue {
        return &self.value;
    }

    pub fn subscribe(&mut self, subscriber: Rc<RefCell<dyn ObjectSubscriber>>) {
        self.subscribers.push(subscriber);
    }
}

pub struct ObjectDictionary {
    dict: HashMap<ObjectKey, Object>,
}

impl ObjectDictionary {
    pub fn new() -> ObjectDictionary {
        ObjectDictionary { dict: HashMap::new() }
    }

    pub fn add(&mut self, index: u16, sub_index: u8, value: ObjectValue) {
        self.dict.insert(ObjectKey { index, sub_index }, Object { index, sub_index, value, subscribers: Vec::new()});
    }

    pub fn write(&mut self, index: u16, sub_index: u8, value: ObjectValue) {
        if let Some(obj) = self.dict.get_mut(&ObjectKey { index, sub_index }) {
            obj.write(value);
        }
    }

    pub fn read(&self, index: u16, sub_index: u8) -> Option<&ObjectValue> {
        match self.dict.get(&ObjectKey { index, sub_index }) {
            Some(obj) => Some(obj.read()),
            None => None
        }
    }

    pub fn subscribe(&mut self, index: u16, sub_index: u8, subscriber: Rc<RefCell<dyn ObjectSubscriber>>) {
        if let Some(obj) = self.dict.get_mut(&ObjectKey { index, sub_index }) {
            obj.subscribe(subscriber);
        }
    }
}