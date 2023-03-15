#[macro_use]
extern crate protocol_derive;

pub trait Packet
where Self: Sized {
    fn decode_packet(opcode: u16, payload: &[u8]) -> Option<Self>;

    fn as_opcode(&self) -> u16;
    fn encode_payload(self) -> Option<Vec<u8>>;
}

#[derive(Debug, PartialEq, Packet)]
enum MyProtocol {
    #[opcode(0x0)]
    VariantA,
    #[opcode(0x1)]
    VariantB {
        name: String
    },
    #[opcode(0x2)]
    VariantC(String, String),

    #[opcode(0x3)]
    VariantD {
        number: u32
    },
    #[opcode(0x4)]
    VariantE(u32)
}

#[test]
fn as_opcode_test() {
    let a = MyProtocol::VariantA;
    let b = MyProtocol::VariantB { name: String::new() }; // dummy string
    let c = MyProtocol::VariantC(String::new(), String::new());

    assert_eq!(a.as_opcode(), 0x0);
    assert_eq!(b.as_opcode(), 0x1);
    assert_eq!(c.as_opcode(), 0x2);
}

#[test]
// Tests that the payload given should be ignored because the variant
// is a unit variant. It does not have any payload to be decoded.
fn ignore_payload_decode_test() {
    let payload: [u8; 3] = [0, 0, 0]; // payload can be anything since it will be ignored
    let ret = MyProtocol::decode_packet(0x0, &payload).unwrap();
    assert_eq!(ret, MyProtocol::VariantA);
}

#[test]
fn empty_payload_encode_test() {
    let expected: [u8; 0] = []; // payload can be anything since it will be ignored
    let payload = MyProtocol::encode_payload(MyProtocol::VariantA).unwrap();
    assert_eq!(payload, expected);
}

#[test]
fn named_decode_payload_test() {
    // a payload of {"name": "loremipsum"}
    let payload: [u8; 17] = [129, 164, 110, 97, 109, 101, 170, 108, 111, 114, 101, 109, 105, 112, 115, 117, 109];

    let ret = MyProtocol::decode_packet(0x1, &payload).unwrap();
    assert_eq!(
        ret,
        MyProtocol::VariantB {
            name: String::from("loremipsum")
        }
    );
}

#[test]
fn unnamed_decode_payload_test() {
    // a payload of ["foo", "bar"]
    let payload: [u8; 9] = [146, 163, 102, 111, 111, 163, 98, 97, 114];

    let ret = MyProtocol::decode_packet(0x2, &payload).unwrap();
    assert_eq!(
        ret,
        MyProtocol::VariantC("foo".to_string(), "bar".to_string())
    );
}

#[test]
fn named_encode_payload_test() {
    let expected: [u8; 17] = [129, 164, 110, 97, 109, 101, 170, 108, 111, 114, 101, 109, 105, 112, 115, 117, 109];

    let packet = MyProtocol::VariantB { name: String::from("loremipsum") };
    let encoded = packet.encode_payload().unwrap();

    assert_eq!(encoded, expected);
}

#[test]
fn unnamed_encode_payload_test() {
    let expected: [u8; 9] = [146, 163, 102, 111, 111, 163, 98, 97, 114];

    let packet = MyProtocol::VariantC("foo".to_string(), "bar".to_string());
    let encoded = packet.encode_payload().unwrap();

    assert_eq!(encoded, expected);
}

#[test]
fn named_encode_u32_payload_test() {
    let expected: [u8; 13] = [129, 166, 110, 117, 109, 98, 101, 114, 206, 0, 1, 226, 64];

    let packet = MyProtocol::VariantD { number: 123456 };
    let encoded = packet.encode_payload().unwrap();

    assert_eq!(encoded, expected);
}

#[test]
fn named_decode_u32_payload_test() {
    let payload: [u8; 13] = [129, 166, 110, 117, 109, 98, 101, 114, 206, 0, 1, 226, 64];
    let ret = MyProtocol::decode_packet(0x3, &payload).unwrap();

    let expected = MyProtocol::VariantD { number: 123456 };

    assert_eq!(ret, expected);
}