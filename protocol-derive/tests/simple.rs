#[macro_use]
extern crate protocol_derive;

pub trait Packet
where Self: Sized {
    fn decode_packet(opcode: u16, payload: &[u8]) -> Option<Self>;

    fn as_opcode(&self) -> u16;
    fn encode_payload(self) -> Vec<u8>;
}

#[derive(Packet)]
enum MyProtocol {
    #[opcode(0x0)]
    VariantA,
    #[opcode(0x1)]
    VariantB,
    #[opcode(0x2)]
    VariantC,
}

#[test]
fn test() {
    // let varianta = <MyProtocol::VariantA as Packet>
}