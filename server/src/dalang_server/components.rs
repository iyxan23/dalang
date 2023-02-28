/// This file is the exports of some pre-made components that the user
/// of this library could use, so they wouldn't need to create their
/// own implementation.

pub mod auth {
    pub use crate::auth::sqlite::SQLiteAuthenticator;
}