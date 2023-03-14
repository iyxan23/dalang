use crate::Packet;

use super::{authentication::{ClientAuthenticationPacket}, Category};

#[test]
fn test_categories() {
    assert_eq!(Category::Authentication as u16, 0x1);
    assert_eq!(Category::User as u16, 0x2);
    assert_eq!(Category::Editor as u16, 0x3);

    assert_eq!(Category::try_from(0x1), Ok(Category::Authentication));
    assert_eq!(Category::try_from(0x2), Ok(Category::User));
    assert_eq!(Category::try_from(0x3), Ok(Category::Editor));
}

#[test]
fn test_client_auth_packet_login() {
    // payload of { username: "lorem", password: "ipsum" },
    // matching with the login packet
    let payload: [u8; 31] = [
        130, 168, 117, 115, 101, 114, 110, 97, 109, 101, 165, 108,
        111, 114, 101, 109, 168, 112, 97, 115, 115, 119, 111, 114,
        100, 165, 105, 112, 115, 117, 109
    ];
    
    let packet = ClientAuthenticationPacket::decode_packet(0x10, &payload)
        .expect("Failed to decode packet");

    assert_eq!(
        packet,
        ClientAuthenticationPacket::Login {
            username: "lorem".to_string(),
            password: "ipsum".to_string()
        }
    )
}