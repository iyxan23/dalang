extern crate protocol_derive;

use protocol_derive::Packet;

#[derive(Debug, Packet)]
enum MyProtocol {
    #[opcode = 0x0]
    Login,
    #[opcode = 0x1]
    Register {
        username: String,
        password: String
    }
}

#[test]
fn test() {

}