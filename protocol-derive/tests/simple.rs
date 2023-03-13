#[macro_use]
extern crate protocol_derive;

pub trait Packet
where Self: Sized {
    fn decode_packet(opcode: u16, payload: &[u8]) -> Option<Self>;

    fn as_opcode(&self) -> u16;
    fn encode_payload(self) -> Vec<u8>;
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
    VariantC,
}

#[test]
fn as_opcode_test() {
    let a = MyProtocol::VariantA;
    let b = MyProtocol::VariantB { name: String::new() }; // dummy string
    let c = MyProtocol::VariantC;

    assert_eq!(a.as_opcode(), 0x0);
    assert_eq!(b.as_opcode(), 0x1);
    assert_eq!(c.as_opcode(), 0x2);
}

#[test]
fn simple_decode_payload_test() {
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