use std::path::PathBuf;

use actix::Handler;
use actix::{Actor, Context};
use pwhash::bcrypt::{hash, verify};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use r2d2_sqlite::rusqlite::{OptionalExtension, params};

use super::Authenticator;
use super::messages as auth_msg;

/// An authenticator with SQLite as backend
pub struct SQLiteAuthenticator {
    db_file: Option<PathBuf>,
    pool: Option<Pool<SqliteConnectionManager>>,
}

impl SQLiteAuthenticator {
    /// Creates a new instace of [`SQLiteAuthenticator`]. Will not do anything to the database
    /// until the actor has been started.
    pub fn new(db_file: PathBuf) -> Self {
        SQLiteAuthenticator {
            db_file: Some(db_file),
            pool: None,
        }
    }

    /// Creates a new instance of [`SQLiteAuthenticator`] with an in-memory database.
    pub fn new_in_memory() -> Self {
        SQLiteAuthenticator {
            db_file: None,
            pool: None,
        }
    }
}

impl Authenticator for SQLiteAuthenticator {}

impl Actor for SQLiteAuthenticator {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        let manager = if let Some(db_file) = &self.db_file {
            // open connection to the database if the db file is present
            SqliteConnectionManager::file(db_file)
        } else {
            // if no db file provided, then use an in-memory database
            SqliteConnectionManager::memory()
        };

        // todo: replace these expect statements to be an error enum
        let pool = r2d2::Pool::new(manager)
            .expect("failed to create connection pool");        

        // create a new table if it doesnt exist
        let conn = pool.get().expect("failed to retrieve connection");
        conn.execute(QUERY_USERS_CREATE, []).expect("failed to create table");

        self.pool = Some(pool);
    }
}

const QUERY_USERS_CREATE: &str = r#"
CREATE TABLE IF NOT EXISTS "users" (
	"uid"	INTEGER NOT NULL UNIQUE,
	"username"	TEXT NOT NULL,
	"password"	TEXT NOT NULL,
	PRIMARY KEY("uid")
)"#;

const QUERY_UID_GET: &str = r#"SELECT * FROM users WHERE uid = ?1;"#;
const QUERY_USERNAME_GET: &str = r#"SELECT * FROM users WHERE username = ?1;"#;

const QUERY_USER_INSERT: &str = r#"INSERT INTO users (uid, username, password) VALUES (?1, ?2, ?3);"#;

impl Handler<auth_msg::Login> for SQLiteAuthenticator {
    type Result = Result<u64, ()>;

    fn handle(&mut self, msg: auth_msg::Login, _ctx: &mut Self::Context) -> Self::Result {
        let conn =
            self.pool
                .as_ref().expect("pool not initialized")
                .clone()
                .get().expect("failed to retrieve connection");
        
        // retrieve the user
        // todo: replace these expect statements to be an error enum
        if let Some((uid, password_hash)) = 
            conn.query_row(
                    QUERY_USERNAME_GET,
                    params![&msg.username],
                    |row| Ok((
                        row.get::<_, u64>("uid")?,
                        row.get::<_, String>("password")?))
                )
                .optional()
                .expect("failed to retrieve user") {

            // check if the password is correct
            // returns `Ok(uid)` if password is correct
            verify(&msg.password, &password_hash)
                .then_some(uid)
                .ok_or(())
        } else {
            // user doesn't exists
            // todo: return a descriptive error
            Err(())
        }
    }
}

impl Handler<auth_msg::Register> for SQLiteAuthenticator {
    type Result = Result<u64, ()>;

    fn handle(&mut self, msg: auth_msg::Register, _ctx: &mut Self::Context) -> Self::Result {
        let conn =
            self.pool
                .as_ref().expect("pool not initialized")
                .clone()
                .get().expect("failed to retrieve connection");

        // check if user already exists
        let changed = conn.execute(QUERY_USERNAME_GET, params![&msg.username])
            .expect("failed to execute query");

        // a user with the same username already exists
        if changed != 0 {
            Err(())?
        }

        // we can then insert our new user
        let uid = rand::random();

        if let Ok(changed) = conn.execute(
            QUERY_USER_INSERT,
            params![uid, &msg.username, hash(&msg.password).map_err(|_| ())?]
        ) {
            assert_eq!(changed, 1);
            Ok(uid)
        } else {
            Err(())
        }
    }
}

impl Handler<auth_msg::GetUser> for SQLiteAuthenticator {
    type Result = Result<String, ()>;

    fn handle(&mut self, msg: auth_msg::GetUser, _ctx: &mut Self::Context) -> Self::Result {
        let conn =
            self.pool
                .as_ref().expect("pool not initialized")
                .clone()
                .get().expect("failed to retrieve connection");
        
        // retrieve the username (for later, we'll have a User model)
        conn.query_row(
            QUERY_UID_GET,
            params![msg.uid],
            |row| Ok(row.get::<_, String>("username")?)
        ).map_err(|_| ())
    }
}

#[cfg(test)]
mod tests {
    use actix::Actor;

    use super::SQLiteAuthenticator;
    use super::super::messages as auth_msg;

    #[actix_rt::test]
    async fn sqlite_simple_auth_test_0() {
        let addr = SQLiteAuthenticator::new_in_memory().start();

        let register_uid = addr.send(auth_msg::Register {
            username: "loremipsum".to_string(),
            password: "1234567890".to_string(),
        }).await.expect("failed send register msg").expect("failed to register");

        let login_uid = addr.send(auth_msg::Login {
            username: "loremipsum".to_string(),
            password: "1234567890".to_string(),
        }).await.expect("failed send login msg").expect("failed to login");

        assert_eq!(register_uid, login_uid);

        let username = addr.send(auth_msg::GetUser {
            uid: register_uid,
        }).await.expect("failed to retrieve username").expect("failed to retrieve username");

        assert_eq!(username, "loremipsum".to_string());
    }
}