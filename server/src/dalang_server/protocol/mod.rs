use rmp::{encode::{ValueWriteError, write_array_len, write_u8, write_str_len, write_str}, decode::{read_marker, read_u32}};
use rmpv::ValueRef;

pub const VERSION: &str = "0.0.1";

pub const VERSION_MAJOR: u8 = 0;
pub const VERSION_MINOR: u8 = 0;
pub const VERSION_PATCH: u8 = 1;

pub const EXTENSIONS: [&str; 0] = [];

#[cfg(test)]
mod tests;

#[macro_use]
mod error;

pub use error::PacketCategoryDecodeError;
pub use error::PacketDecodeError;
pub use error::PayloadDecodeError;

// maybe cache this in some way? I'm too lazy to use `lazy_static` (pun intended)
pub fn protocol_version_packet() -> Result<Vec<u8>, ValueWriteError> {
    let mut buffer = Vec::new();

    write_array_len(&mut buffer, 2)?;

    write_array_len(&mut buffer, 3)?;
    write_u8(&mut buffer, VERSION_MAJOR)?;
    write_u8(&mut buffer, VERSION_MINOR)?;
    write_u8(&mut buffer, VERSION_PATCH)?;

    write_array_len(&mut buffer, EXTENSIONS.len() as u32)?;

    for extension in EXTENSIONS {
        write_str_len(&mut buffer, extension.len() as u32)?;
        write_str(&mut buffer, extension)?;
    }

    Ok(buffer)
}

pub enum ClientPacket {
    Authentication(authentication::ClientAuthenticationPacket),
    User(user::ClientUserPacket),
    Editor(editor::ClientEditorPacket)
}

impl TryFrom<&[u8]> for ClientPacket {
    type Error = PacketDecodeError;

    fn try_from(mut value: &[u8]) -> Result<Self, Self::Error> {
        // the client packet is an array of two items:
        // 0 - the opcode
        // 1 - an object of payload, may be null

        // read an array of two items
        let rmp::Marker::FixArray(2) = read_marker(&mut value)? else {
            Err(PacketDecodeError::InvalidStructure)?
        };

        let opcode = read_u32(&mut value)?;
        let category = (opcode >> 16) as u16; 

        let Ok(category): Result<Category, _> = category.try_into() else {
            // unknown category
            Err(PacketDecodeError::UnknownCategory { given_category: category })?
        };

        let opcode = (opcode & 0xffff) as u16;

        Ok(match category {
            Category::Authentication => 
                ClientPacket::Authentication(
                    authentication::ClientAuthenticationPacket
                        ::decode_from(opcode, &value)

                        // include category as needed by the From trait
                        .map_err(|e| (category, e))?
                ),
            Category::User => todo!(),
            Category::Editor => todo!(),
        })
    }
}

impl TryInto<Vec<u8>> for ClientPacket {
    type Error = ValueWriteError;

    fn try_into(self) -> Result<Vec<u8>, Self::Error> {
        // seems a bit of a waste to implement
        // servers don't need to serialize client packets
        unimplemented!()
    }
}

pub enum ServerPacket {
    Authentication(authentication::ServerAuthenticationPacket),
    User(user::ServerUserPacket),
    Editor(editor::ServerEditorPacket)
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
#[repr(u16)]
pub enum Category {
    Authentication = 0x01,
    User = 0x02,
    Editor = 0x03,
}

impl TryFrom<u16> for Category {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Ok(match value {
            0x01 => Category::Authentication,
            0x02 => Category::User,
            0x03 => Category::Editor,

            _ => Err(())?,
        })
    }
}

// === Trait PacketDecoder
// This trait should be implemented of all packet structs.
trait PacketDecoder
where Self: Sized {
    type Opcode: Into<u16> + TryFrom<u16>;

    fn decode_from(opcode: u16, payload: &[u8]) -> Result<Self, PacketCategoryDecodeError<Self::Opcode>>;
}

// === Trait PayloadDecoder
// This trait should be implemented for payload packet structs.
trait PayloadDecoder
where Self: Sized {
    type Opcode: Into<u16> + TryFrom<u16>;

    fn decode_payload(opcode: Self::Opcode, payload: &[u8]) -> Result<Option<Self>, PayloadDecodeError>;
}

macro_rules! decode_payload {
    { $payload:ident; $typ:ty { $($str_names:expr => $names:ident),* $(,)? } } => {
        let payload = decode_payload($payload)
            .ok_or(PayloadDecodeError::InvalidPayload)?;

        // we loop over the values, check needed ones and skip other fields
        //
        // this is to future-proof where maybe recent versions might have
        // some other fields
        let mut map =
            payload
                .into_iter()
                .filter_map(|(key, val)| {
                    let ValueRef::String(key) = key else { None? };
                    key.into_str().map(|s| (s, val))
                })
                .try_fold::<_, _, Result<_, PayloadDecodeError>>(
                    HashMap::new(),
                    |mut acc, (key, val)| {

                    match key {
                        $(
                            $str_names => {
                                acc.insert($str_names, get_str(val).ok_or(PayloadDecodeError::InvalidPayload)?);
                            }
                        )*
                        _ => {},
                    }

                    Ok(acc)
                })?;

        $ty {
            $(
                $names: map.remove($str_names).ok_or(PayloadDecodeError::InvalidPayload)?.to_owned(),
            )*
        }
    };
}

// +===========================+
// |     Packet Categories     |
// +===========================+
//
// These modules includes opcodes of each categories, both for the server and client.
//
// There are three categories as defined in the `Category` enum:
// - Authentication: 0x1
// - User: 0x2
// - Editor: 0x3

// >> Authentication Packet Category
pub mod authentication {
    use std::collections::HashMap;

    use rmpv::ValueRef;

    use super::{PacketCategoryDecodeError, PacketDecoder, PayloadDecodeError, decode_payload, get_str, PayloadDecoder};

    #[derive(Clone, Debug, PartialEq)]
    pub struct ClientAuthenticationPacket {
        pub opcode: ClientOpcode,
        pub payload: Option<ClientPacketPayload>,
    }

    impl PacketDecoder for ClientAuthenticationPacket {
        type Opcode = ClientOpcode;

        fn decode_from(opcode: u16, payload: &[u8]) -> Result<Self, PacketCategoryDecodeError<Self::Opcode>> {
            let opcode = ClientOpcode::try_from(opcode)
                .map_err(|_| PacketCategoryDecodeError::UnknownOpcode { opcode })?;

            Ok(ClientAuthenticationPacket {
                opcode,
                payload: ClientPacketPayload::decode_payload(opcode, payload)
                    .map_err(|err| (opcode, err))?
            })
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct ServerAuthenticationPacket {
        pub opcode: ServerOpcode,
        pub payload: Option<ServerPacketPayload>,
    }

    #[repr(u16)]
    #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
    pub enum ClientOpcode {
        SuccessResp = 0x00,

        Login = 0x10, // Data: { username: str, password: str }
        LoginWithToken = 0x11, // Data: { token: str }
        Register = 0x20, // Data: { username: str, password: str }
        RegisterCheckEnabled = 0x21, // Response: 0x21 or 0x00

        UsernameCheckExists = 0xf0, // Response: 0x00 or 0x02

        Logout = 0x00ff,
    }

    impl Into<u16> for ClientOpcode {
        fn into(self) -> u16 { self as u16 }
    }
    
    // todo: make these to be generated automatically using macros
    impl TryFrom<u16> for ClientOpcode {
        type Error = ();

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            Ok(match value {
                0x00 => ClientOpcode::SuccessResp,

                0x10 => ClientOpcode::Login,
                0x11 => ClientOpcode::LoginWithToken,
                0x20 => ClientOpcode::Register,
                0x21 => ClientOpcode::RegisterCheckEnabled,

                0xf0 => ClientOpcode::UsernameCheckExists,

                0x00ff => ClientOpcode::Logout,
                _ => Err(())?
            })
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum ClientPacketPayload {
        Login {
            username: String,
            password: String,
        },
        LoginWithToken {
            token: String
        },
        Register {
            username: String,
            password: String
        },
    }

    impl PayloadDecoder for ClientPacketPayload {
        type Opcode = ClientOpcode;

        fn decode_payload(opcode: Self::Opcode, payload: &[u8]) -> Result<Option<Self>, PayloadDecodeError> {
            Ok(Some(match opcode {
                ClientOpcode::Login => {
                    decode_payload! { payload;
                        Self::Login {
                            "username" => username,
                            "password" => password,
                        }
                    }
                },
                ClientOpcode::LoginWithToken => {
                    decode_payload! { payload;
                        Self::LoginWithToken {
                            "token" => token
                        }
                    }
                },
                ClientOpcode::Register => {
                    decode_payload! { payload;
                        Self::Register {
                            "username" => username,
                            "password" => password,
                        }
                    }
                },
                _ => return Ok(None)
            }))
        }
    }

    #[repr(u16)]
    #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
    pub enum ServerOpcode {
        SuccessResp = 0x00,
        
        LoginFailedInvalidUsernameWrongPassword = 0x10,
        LoginFailedTokenExpired = 0x11,
        LoginSuccess = 0x12, // Data: { token: str }

        RegisterFailedUsernameTaken = 0x20,
        RegisterFailedFeatureDisabled = 0x21,

        ErrorAlreadyLoggedIn = 0xffff,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum ServerPacketPayload {
        LoginSuccess {
            token: String,
        }
    }
}

// >> User Packet Category
pub mod user {
    #[derive(Clone, Debug, PartialEq)]
    pub struct ClientUserPacket {
        pub opcode: ClientOpcode,
        pub payload: Option<ClientPacketPayload>,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct ServerUserPacket {
        pub opcode: ServerOpcode,
        pub payload: Option<ServerPacketPayload>,
    }

    #[repr(u16)]
    #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
    pub enum ClientOpcode {
        SuccessResp = 0x00,

        GetUsername = 0x01, // Response: Server 0x00

        RetrieveProjects = 0x10, // Response: Server 0x01
        RetrieveProjectsPaged = 0x11, // Data: { offset: u32, count: u32 }, Response: Server 0x01
        RetrieveProjectsTotal = 0x12, // Response: Server 0x11
        RetrieveProjectImage = 0x13, // Data: { imgid: u32 } Response: Server 0x12

        OpenProject = 0x1f, // Response: Server 0x00 (editor category (0x3))
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum ClientPacketPayload {
        RetrieveProjectsPaged {
            offset: u32,
            count: u32,
        },
        RetrieveProjectImage {
            imgid: u32
        }
    }


    #[repr(u16)]
    #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
    pub enum ServerOpcode {
        SuccessResp = 0x00,

        UsernameResp = 0x01, // Data: { username: str }

        ProjectsListResp = 0x10, // Data: { projects: [{ id: u32, title: str, lastedit: u64, created: u64, imgid: u32 }] }
        ProjectsTotalResp = 0x11, // Data: { total: u32 }
        ProjectImageResp = 0x12, // Data: { data: [u8] }

        ErrorNotAuthenticated = 0xffff,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum ServerPacketPayload {
        UsernameResp {
            username: String,
        },
        ProjectsListResp {
            projects: Vec<ProjectData>,
        },
        ProjectsTotalResp {
            total: u32,
        },
        ProjectImageResp {
            data: Vec<u8>,
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct ProjectData {
        pub id: u32,
        pub title: String,
        pub lastedit: u64,
        pub created: u64,
        pub imgid: u32,
    }
}

// >> Editor Packet Category
pub mod editor {
    #[derive(Clone, Debug, PartialEq)]
    pub struct ClientEditorPacket {
        pub opcode: ClientOpcode,
        pub payload: Option<ClientPacketPayload>,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct ServerEditorPacket {
        pub opcode: ServerOpcode,
        pub payload: Option<ServerPacketPayload>,
    }

    #[repr(u16)]
    #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
    pub enum ClientOpcode {
        SuccessResp = 0x00,
    }
    
    #[derive(Clone, Debug, PartialEq)]
    pub enum ClientPacketPayload {
    }

    #[repr(u16)]
    #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
    pub enum ServerOpcode {
        SuccessResp = 0x00,
    }
    
    #[derive(Clone, Debug, PartialEq)]
    pub enum ServerPacketPayload {
    }
}

// ======== Utilities ========

// use rmpv to decode the payload to a map
/// Decode a payload into a [`Vec<(ValueRef, ValueRef)>`], otherwise return an `Err`.
pub(crate) fn decode_payload(mut payload: &[u8]) -> Option<Vec<(ValueRef, ValueRef)>> {
    rmpv::decode::read_value_ref(&mut payload)
        .ok()
        .map(|v| match v {
            ValueRef::Map(map) => Some(map),
            _ => None,
        })
        .flatten()
}

/// Try to get a [`&str`] from a [`ValueRef`], otherwise return `None`.
pub(crate) fn get_str<'a>(val: ValueRef<'a>) -> Option<&'a str> {
    let ValueRef::String(val) = val else { None? };
    let Some(val) = val.into_str() else { None? };

    Some(val)
}

// ======== /Utilities ========