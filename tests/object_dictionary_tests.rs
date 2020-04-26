extern crate canopen_rs;

use canopen_rs::od::{ObjectDictionary, ObjectValue};

#[test]
fn test_object_dictionary_read() {
    let mut od = ObjectDictionary::new();
    od.add(0x1000, 0x00, ObjectValue::Unsigned32(0x1234));
    match od.read(0x1000, 0x00) {
        Some(x) => match x {
            ObjectValue::Unsigned32(y) => assert_eq!(*y, 0x1234),
            _ => assert!(false)
        },
        None => assert!(false)
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
            _ => assert!(false)
        },
        None => assert!(false)
    }
}

#[test]
fn test_object_dictionary_write_value_type_mismatch() {
    let mut od = ObjectDictionary::new();
    od.add(0x1000, 0x00, ObjectValue::Unsigned32(0x4000));
    od.write(0x1000, 0x00, ObjectValue::Unsigned8(0xFF));
    match od.read(0x1000, 0x00) {
        Some(x) => match x {
            ObjectValue::Unsigned32(y) => assert_eq!(*y, 0x4000),
            _ => assert!(false)
        },
        None => assert!(false)
    }
}