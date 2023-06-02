use dotenv::dotenv;
use mongodb::bson::doc;
use mongodb::{Client, IndexModel};
use pwhash::{bcrypt, sha256_crypt, sha512_crypt};
use rand::random;
use tonic::{transport::Server, Request, Response, Status};

pub mod auth {
    tonic::include_proto!("auth");
}

mod models;

use auth::authentication_service_server::{AuthenticationService, AuthenticationServiceServer};
use auth::*;

const DATABASE_NAME: &str = "auth";
const USERS_COLLECTION_NAME: &str = "users";
const TOKENS_COLLECTION_NAME: &str = "tokens";

#[derive(Debug)]
pub struct Service {
    mongodb: Client,
}

#[tonic::async_trait]
impl AuthenticationService for Service {
    async fn authenticate(
        &self,
        request: Request<Credentials>,
    ) -> Result<Response<AuthenticationResult>, Status> {
        todo!()
    }

    async fn create_account(
        &self,
        request: Request<Credentials>,
    ) -> Result<Response<AuthenticationResult>, Status> {
        let Credentials { username, password } = request.into_inner();

        // check if username exists
        if let Some(_existing_user) = self
            .mongodb
            .database(DATABASE_NAME)
            .collection::<models::User>(USERS_COLLECTION_NAME)
            .find_one(doc! { "username": username.clone() }, None)
            .await
            .map_err(|mongo_err| {
                log::error!(
                    "error while checking for user with username `{username}`: {}",
                    mongo_err.to_string()
                );
                Status::internal("unable to create account")
            })?
        {
            // user with username already exists
            return Ok(Response::new(AuthenticationResult {
                status: 0,
                error_message: Some("A user with the given username already exists".to_string()),
                token: None,
            }));
        }

        let hashed_password = bcrypt::hash_with(
            bcrypt::BcryptSetup {
                cost: Some(12),
                ..Default::default()
            },
            password,
        )
        .map_err(|bcrypt_err| {
            log::error!("bcrypt error while hashing username `{username}`: {bcrypt_err:?}");
            Status::internal("unable to create account")
        })?;

        // create a user id from a sha256 hash of a random array
        #[allow(deprecated)]
        let user_id = sha256_crypt::hash(random::<[u8; 32]>()).map_err(|sha_err| {
            log::error!("bcrypt error while hashing username `{username}`: {sha_err:?}");
            Status::internal("unable to create account")
        })?;

        self.mongodb
            .database(DATABASE_NAME)
            .collection::<models::User>(USERS_COLLECTION_NAME)
            .insert_one(
                models::User {
                    id: user_id.clone(),
                    username,
                    hashed_password,
                },
                None,
            )
            .await
            .map_err(|mongo_err| {
                log::error!(
                    "unable to insert user data `{user_id}` into mongo: {}",
                    mongo_err.to_string()
                );
                Status::internal("unable to create account")
            })?;

        let new_token = sha512_crypt::hash(random::<[u8; 32]>()).map_err(|sha_err| {
            log::error!("bcrypt error while hashing username `{user_id}`: {sha_err:?}");
            Status::internal("unable to create account")
        })?;

        // create token
        self.mongodb
            .database(DATABASE_NAME)
            .collection::<models::Token>(TOKENS_COLLECTION_NAME)
            .insert_one(
                models::Token {
                    token: new_token.clone(),
                    user_id: user_id.clone(),

                    // first access token should be pretty short-lived (a day)
                    // then subsequent refresh tokens will live longer
                    expire_until: std::time::Instant::now()
                        .checked_add(std::time::Duration::from_secs(86400))
                        .expect("the expire time to be within the bounds of std::time::Instant")
                        .elapsed()
                        .as_secs(),
                },
                None,
            )
            .await
            .map_err(|mongo_err| {
                log::error!(
                    "unable to insert first access token for the newly-created account with id of `{user_id}`: {mongo_err:?}"
                );
                Status::internal("unable to create account")
            })?;

        return Ok(Response::new(AuthenticationResult {
            status: 0,
            error_message: None,
            token: Some(Token { token: new_token }),
        }));
    }

    async fn get_user_id(&self, request: Request<Token>) -> Result<Response<UserId>, Status> {
        todo!()
    }

    async fn check_validity(
        &self,
        request: Request<Token>,
    ) -> Result<Response<TokenValidity>, Status> {
        todo!()
    }

    async fn refresh_token(
        &self,
        request: Request<Token>,
    ) -> Result<Response<AuthenticationResult>, Status> {
        todo!()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    dotenv().ok();

    let grpc_listen_addr = std::env::var("AUTHSERVICE_SERVE_ON")
        .expect("a valid `AUTHSERVICE_SERVE_ON` env variable")
        .parse()?;

    let mongo_uri = std::env::var("AUTHSERVICE_MONGOD_URI")
        .expect("a valid `AUTHSERVICE_MONGO_URI` env variable");

    let mongo_client = Client::with_uri_str(mongo_uri)
        .await
        .expect("to connect to mongodb from the given `AUTHSERVICE_MONGO_URI` env variable");

    mongo_client
        .database(DATABASE_NAME)
        .collection::<models::User>(USERS_COLLECTION_NAME)
        .create_index(
            IndexModel::builder().keys(doc! { "username": 1 }).build(),
            None,
        )
        .await
        .expect("create index to work");

    mongo_client
        .database(DATABASE_NAME)
        .collection::<models::Token>(TOKENS_COLLECTION_NAME)
        .create_index(
            IndexModel::builder()
                .keys(doc! { "token": 1, "user_id": 1 })
                .build(),
            None,
        )
        .await
        .expect("create index to work");

    let service = Service {
        mongodb: mongo_client,
    };

    Server::builder()
        .add_service(AuthenticationServiceServer::new(service))
        .serve(grpc_listen_addr)
        .await?;

    Ok(())
}
