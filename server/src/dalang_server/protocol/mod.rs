use rmp::encode::{ValueWriteError, write_array_len, write_u8, write_str_len, write_str};

pub const VERSION: &str = "0.0.1";

pub const VERSION_MAJOR: u8 = 0;
pub const VERSION_MINOR: u8 = 0;
pub const VERSION_PATCH: u8 = 1;

pub const EXTENSIONS: [&str; 0] = [];

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

impl From<&[u8]> for ClientPacket {
    fn from(value: &[u8]) -> Self {
        todo!()
    }
}

impl Into<Vec<u8>> for ClientPacket {
    fn into(self) -> Vec<u8> {
        todo!()
    }
}

pub enum ServerPacket {
    Authentication(authentication::ServerAuthenticationPacket),
    User(user::ServerUserPacket),
    Editor(editor::ServerEditorPacket)
}


impl From<&[u8]> for ServerPacket {
    fn from(value: &[u8]) -> Self {
        todo!()
    }
}

impl Into<Vec<u8>> for ServerPacket {
    fn into(self) -> Vec<u8> {
        todo!()
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
#[repr(u16)]
pub enum Category {
    Authentication = 0x01,
    User = 0x02,
    Editor = 0x03,
}

pub mod authentication {
    #[derive(Clone, Debug, PartialEq)]
    pub struct ClientAuthenticationPacket {
        pub opcode: ClientOpcode,
        pub payload: Option<ClientPacketPayload>,
    }

    impl From<&[u8]> for ClientAuthenticationPacket {
        fn from(value: &[u8]) -> Self {
            todo!()
        }
    }
    
    impl Into<Vec<u8>> for ClientAuthenticationPacket {
        fn into(self) -> Vec<u8> {
            todo!()
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct ServerAuthenticationPacket {
        pub opcode: ServerOpcode,
        pub payload: Option<ServerPacketPayload>,
    }

    impl From<&[u8]> for ServerAuthenticationPacket {
        fn from(value: &[u8]) -> Self {
            todo!()
        }
    }
    
    impl Into<Vec<u8>> for ServerAuthenticationPacket {
        fn into(self) -> Vec<u8> {
            todo!()
        }
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

pub mod user {
    #[derive(Clone, Debug, PartialEq)]
    pub struct ClientUserPacket {
        pub opcode: ClientOpcode,
        pub payload: Option<ClientPacketPayload>,
    }

    impl From<&[u8]> for ClientUserPacket {
        fn from(value: &[u8]) -> Self {
            todo!()
        }
    }
    
    impl Into<Vec<u8>> for ClientUserPacket {
        fn into(self) -> Vec<u8> {
            todo!()
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct ServerUserPacket {
        pub opcode: ServerOpcode,
        pub payload: Option<ServerPacketPayload>,
    }

    impl From<&[u8]> for ServerUserPacket {
        fn from(value: &[u8]) -> Self {
            todo!()
        }
    }
    
    impl Into<Vec<u8>> for ServerUserPacket {
        fn into(self) -> Vec<u8> {
            todo!()
        }
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

pub mod editor {
    #[derive(Clone, Debug, PartialEq)]
    pub struct ClientEditorPacket {
        pub opcode: ClientOpcode,
        pub payload: Option<ClientPacketPayload>,
    }


    impl From<&[u8]> for ClientEditorPacket {
        fn from(value: &[u8]) -> Self {
            todo!()
        }
    }
    
    impl Into<Vec<u8>> for ClientEditorPacket {
        fn into(self) -> Vec<u8> {
            todo!()
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct ServerEditorPacket {
        pub opcode: ServerOpcode,
        pub payload: Option<ServerPacketPayload>,
    }

    impl From<&[u8]> for ServerEditorPacket {
        fn from(value: &[u8]) -> Self {
            todo!()
        }
    }
    
    impl Into<Vec<u8>> for ServerEditorPacket {
        fn into(self) -> Vec<u8> {
            todo!()
        }
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