use std::{path::PathBuf, collections::HashMap, net::ToSocketAddrs};

use actix::{Actor, Addr};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, middleware};
use actix_web_actors::ws;
use server::DalangServer;

mod auth;
mod session;
mod storage;
mod protocol;

pub mod components;

async fn ws_endpoint<AuthActor: auth::Authenticator>(
    req: HttpRequest,
    stream: web::Payload
) -> Result<HttpResponse, Error> {
    let Some(server) = req.app_data::<ServerState<AuthActor>>() else {
        return HttpResponse::InternalServerError().await;
    };

    // todo: share the ThreadRng for a bit better performance. But it's a bit tricky since
    //       ThreadRng is !Send and !Sync. Interior mutability might work but that might lead
    //       to some messy code.
    ws::WsResponseBuilder::new(
        session::Session {
            id: rand::random(),
            server: server.server.clone()
        },
        &req, stream
    ).protocols(&["dalang"]).start()
}

struct ServerState<AuthActor: auth::Authenticator> {
    server: Addr<DalangServer<AuthActor>>,
}

/// Starts the dalang server.
/// 
/// # Arguments
/// 
/// * `endpoint` - An endpoint where the websocket server will be run. Will use `/` if not specified.
/// * `serve_static` - Tell the library to serve static files in this directory. Will not serve any static files if not specified.
/// * `create_auth` - The function to construct an Authenticator of the given `AuthActor` type parameter.
pub async fn start<AuthActor, CreateAuthFn, S: ToSocketAddrs>(
    endpoint: Option<String>,
    serve_static: Option<PathBuf>,
    create_auth: CreateAuthFn,
    addr: S
) -> std::io::Result<()>

where
    AuthActor: auth::Authenticator,
    CreateAuthFn: FnOnce() -> AuthActor
{
    let authenticator = create_auth();
    let auth_addr = authenticator.start();

    let server =
        DalangServer::<AuthActor> {
            authenticator: auth_addr,
            storages: HashMap::new(),
        };

    let server_addr = server.start();

    let server =
        web::Data::new(ServerState::<AuthActor> { server: server_addr });

    HttpServer::new(move || {
        let mut app = App::new()
            .app_data(server.clone())
            .wrap(middleware::Logger::default())
            .route(
                endpoint.clone().unwrap_or("/".to_string()).as_str(),
                web::get().to(ws_endpoint::<AuthActor>)
            );

        if let Some(static_files) = &serve_static {
            app = app.service(actix_files::Files::new("/", static_files));
        }

        app
    })
    .bind(addr)?
    .run()
    .await
}

mod server {
    use std::{collections::HashMap, marker::PhantomData};

    use actix::{Addr, Actor, Context, Message, Handler};

    use crate::auth::Authenticator;

    use super::storage::Storage;

    #[derive(Debug, Clone)]
    pub struct DalangServer<AuthActor: Authenticator> {
        pub authenticator: Addr<AuthActor>,
        pub storages: HashMap<u64, Addr<Storage>>,
    }

    impl<A: Authenticator> Actor for DalangServer<A> {
        type Context = Context<Self>;
    }

    #[derive(Debug)]
    pub struct GetAuthenticator<AuthActor: Authenticator>(pub PhantomData<AuthActor>);

    impl<AuthActor: Authenticator> Message for GetAuthenticator<AuthActor> {
        type Result = Addr<AuthActor>;
    }

    impl<AuthActor: Authenticator> Handler<GetAuthenticator<AuthActor>> for DalangServer<AuthActor> {
        type Result = Addr<AuthActor>;

        fn handle(&mut self, _msg: GetAuthenticator<AuthActor>, _ctx: &mut Self::Context) -> Self::Result {
            self.authenticator.clone()
        }
    }
}