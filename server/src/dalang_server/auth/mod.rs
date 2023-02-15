use actix::{Actor, Handler};

pub mod sqlite;

// An authenticator is an actor that handles Login and Register messages
pub trait Authenticator:
    Actor
    + Handler<messages::Login>
    + Handler<messages::Register>
    + Handler<messages::GetUser>
{}

pub mod messages {
    use actix::Message;

    // Login message, results in the UID of the user
    pub struct Login {
        pub username: String,
        pub password: String,
    }

    impl Message for Login {
        // todo: a custom error type
        type Result = Result<u64, ()>;
    }

    // Register message, returns the UID of the new user
    pub struct Register {
        pub username: String,
        pub password: String
    }

    impl Message for Register {
        // todo: a custom error type
        type Result = Result<u64, ()>;
    }

    // Retrieves a user data, currently it only returns the username
    pub struct GetUser {
        pub uid: u64
    }

    impl Message for GetUser {
        // todo: custom error type
        type Result = Result<String, ()>;
    }
}