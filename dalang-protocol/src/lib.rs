use rmp::{
    decode::{read_marker, read_u32},
    encode::{write_array_len, write_str, write_str_len, write_u8, ValueWriteError},
};

// might be a good idea to use the version specified on the cargo manifest file
// but it'd be a problem converting it into these MAJOR, MINOR, and PATCH vars
pub const VERSION: &'static str = "0.0.1";

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

// maybe cache this in some way? I'm too lazy to use `lazy_static` (pun intended)
/// Generates a packet that contains the version information of the protocol
/// used at the start of handshake between the server and the client.
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
    Editor(editor::ClientEditorPacket),
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
            Category::Authentication => ClientPacket::Authentication(
                authentication::ClientAuthenticationPacket::decode_packet(opcode, &value)
                    .ok_or_else(|| PacketDecodeError::InvalidPayload { category, opcode })?,
            ),
            Category::User => ClientPacket::User(
                user::ClientUserPacket::decode_packet(opcode, &value)
                    .ok_or_else(|| PacketDecodeError::InvalidPayload { category, opcode })?,
            ),
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
    Editor(editor::ServerEditorPacket),
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

/// The trait that will be implemented in every packets
pub trait Packet
where
    Self: Sized,
{
    fn decode_packet(opcode: u16, payload: &[u8]) -> Option<Self>;

    fn as_opcode(&self) -> u16;
    fn encode_payload(self) -> Option<Vec<u8>>;
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
    use super::Packet;
    use protocol_derive::Packet;

    #[derive(Debug, Clone, PartialEq, Packet)]
    pub enum ClientAuthenticationPacket {
        #[opcode(0x00)]
        SuccessResp,
        #[opcode(0x10)]
        Login {
            #[from_cloned]
            username: String,
            #[from_cloned]
            password: String,
        },
        #[opcode(0x11)]
        LoginWithToken {
            #[from_cloned]
            token: String,
        },
        #[opcode(0x20)]
        Register {
            #[from_cloned]
            username: String,
            #[from_cloned]
            password: String,
        },
        #[opcode(0x21)]
        RegisterCheckEnabled,
        #[opcode(0xf0)]
        UsernameCheckExists,
        #[opcode(0x00ff)]
        Logout,
    }

    #[derive(Debug, Clone, PartialEq, Packet)]
    pub enum ServerAuthenticationPacket {
        #[opcode(0x00)]
        SuccessResp,
        #[opcode(0x10)]
        LoginFailedInvalidUsernameWrongPassword,
        #[opcode(0x11)]
        LoginFailedTokenExpired,
        #[opcode(0x12)]
        LoginSuccess {
            #[from_cloned]
            token: String,
        },
        #[opcode(0x20)]
        RegisterFailedUsernameTaken,
        #[opcode(0x21)]
        RegisterFailedFeatureDisabled,
        #[opcode(0xffff)]
        ErrorAlreadyLoggedIn,
    }
}

// >> User Packet Category
pub mod user {
    use super::Packet;
    use protocol_derive::Packet;

    #[derive(Debug, Clone, PartialEq, Packet)]
    pub enum ClientUserPacket {
        #[opcode(0x00)]
        SuccessResp,

        #[opcode(0x01)]
        GetUsername,

        #[opcode(0x10)]
        RetrieveProjects,
        #[opcode(0x11)]
        RetrieveProjectsPaged { offset: u64, count: u64 },
        #[opcode(0x12)]
        RetrieveProjectsTotal,
        #[opcode(0x13)]
        RetrieveProjectImage { imgid: u64 },

        #[opcode(0x1f)]
        OpenProject,
    }

    #[derive(Debug, Clone, PartialEq, Packet)]
    pub enum ServerUserPacket {
        #[opcode(0x00)]
        SuccessResp,

        #[opcode(0x01)]
        UsernameResp {
            #[from_cloned]
            username: String,
        },

        #[opcode(0x10)]
        ProjectsListResp {
            // projects: ProjectData,
            _temp: u64,
        },
        #[opcode(0x11)]
        ProjectsTotalResp { total: u64 },
        #[opcode(0x12)]
        ProjectImageResp {
            #[from_cloned]
            data: Vec<u8>,
        },

        #[opcode(0xffff)]
        ErrorNotAuthenticated,
    }

    fn decode_u8_array(val: rmpv::ValueRef) -> Option<Vec<u8>> {
        todo!()
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct ProjectData {
        pub id: u32,
        pub title: String,
        pub lastedit: u64,
        pub created: u64,
        pub imgid: u32,
    }

    impl Into<rmpv::Value> for ProjectData {
        fn into(self) -> rmpv::Value {
            todo!()
        }
    }

    fn decode_project_data(val: rmpv::ValueRef) -> Option<ProjectData> {
        todo!()
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
    pub enum ClientPacketPayload {}

    #[repr(u16)]
    #[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
    pub enum ServerOpcode {
        SuccessResp = 0x00,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub enum ServerPacketPayload {}
}
