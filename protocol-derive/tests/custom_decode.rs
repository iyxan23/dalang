#[macro_use]
extern crate protocol_derive;

pub trait Packet
where
    Self: Sized,
{
    fn decode_packet(opcode: u16, payload: &[u8]) -> Option<Self>;

    fn as_opcode(&self) -> u16;
    fn encode_payload(self) -> Option<Vec<u8>>;
}

#[derive(Debug, PartialEq, Packet)]
enum MyProtocol {
    #[opcode(0x00)]
    Payload {
        #[from_cloned]
        data: MyDataStruct,
    },
    #[opcode(0x01)]
    TuplePayload(#[from_cloned] MyDataStruct),
}

fn decode_my_data_struct(_val: rmpv::ValueRef) -> Option<MyDataStruct> {
    todo!()
}

#[derive(Debug, PartialEq)]
struct MyDataStruct {
    number: u32,
    numbers: Vec<u32>,
    text: String,
}

impl From<MyDataStruct> for rmpv::Value {
    fn from(value: MyDataStruct) -> Self {
        todo!()
    }
}

impl TryFrom<rmpv::Value> for MyDataStruct {
    type Error = std::io::Error;

    fn try_from(value: rmpv::Value) -> Result<Self, Self::Error> {
        todo!()
    }
}
