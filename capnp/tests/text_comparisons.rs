#![cfg(feature = "alloc")]

use capnp::{message, text};

#[test]
pub fn text_comparisons() {
    let mut msg1 = message::Builder::new_default();
    let mut msg2 = message::Builder::new_default();

    msg1.set_root::<text::Reader>("abcde".into()).unwrap();
    msg2.set_root::<text::Reader>("fghij".into()).unwrap();

    let str1 = msg1.get_root_as_reader::<text::Reader>().unwrap();
    let str2 = msg2.get_root_as_reader::<text::Reader>().unwrap();

    assert!(str1 < str2);
    assert!(str1 < "zzzz");
    assert!("aaaa" < str2);
    assert_eq!(str1, "abcde");
    assert_eq!("fghij", str2);
    assert_ne!(str1, str2);
}
