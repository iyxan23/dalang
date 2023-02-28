use actix::{Actor, Handler, Context};

pub mod sqlite;

// An authenticator is an actor that handles Login and Register messages
pub trait Authenticator:
    Actor<Context = Context<Self>>
    + Handler<messages::Login>
    + Handler<messages::Register>
    + Handler<messages::GetUser>

    + Send + Sync
{}

pub mod messages {
    use actix::Message;

    // Login message, results in the UID of the user
    #[derive(Message)]
    #[rtype("Result<u64, ()>")]
    pub struct Login {
        pub username: String,
        pub password: String,
    }

    // Register message, returns the UID of the new user
    #[derive(Message)]
    #[rtype("Result<u64, ()>")]
    pub struct Register {
        pub username: String,
        pub password: String
    }

    // Retrieves a user data, currently it only returns the username
    #[derive(Message)]
    #[rtype("Result<String, ()>")]
    pub struct GetUser {
        pub uid: u64
    }
}