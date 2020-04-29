extern crate canopen_rs;

use std::cell::RefCell;
use std::rc::Rc;

use canopen_rs::od::{ObjectDictionary, ObjectSubscriber, ObjectValue};

struct MySubscriber {
    pub value: i32,
}

impl ObjectSubscriber for MySubscriber {
    fn object_updated(&mut self, _index: u16, _sub_index: u8, value: &ObjectValue) {
        match value {
            ObjectValue::Integer32(v) => self.value = *v,
            _ => {}
        }
    }
}

#[test]
fn test_object_dictionary_read() {
    let mut od = ObjectDictionary::new();
    od.add(0x1000, 0x00, ObjectValue::Unsigned32(0x1234));
    match od.read(0x1000, 0x00) {
        Some(x) => match x {
            ObjectValue::Unsigned32(y) => assert_eq!(*y, 0x1234),
            _ => assert!(false),
        },
        None => assert!(false),
    }
}

#[test]
fn test_object_dictionary_write_same_value_type() {
    let mut od = ObjectDictionary::new();
    od.add(0x1000, 0x00, ObjectValue::Unsigned32(0x4000));
    od.write(0x1000, 0x00, ObjectValue::Unsigned32(0x8200));
    match od.read(0x1000, 0x00) {
        Some(x) => match x {
            ObjectValue::Unsigned32(y) => assert_eq!(*y, 0x8200),
            _ => assert!(false),
        },
        None => assert!(false),
    }
}

#[test]
fn test_object_dictionary_subscription() {
    let mut od = ObjectDictionary::new();
    od.add(0x1016, 0x01, ObjectValue::Integer32(410));

    let subscriber = Rc::new(RefCell::new(MySubscriber { value: 0 }));
    od.subscribe(0x1016, 0x01, subscriber.clone());

    od.write(0x1016, 0x01, ObjectValue::Integer32(360));

    assert_eq!(subscriber.borrow().value, 360);
}
