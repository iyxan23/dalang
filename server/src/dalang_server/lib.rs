use std::path::PathBuf;

use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer, middleware};
use actix_web_actors::ws;

mod auth;
mod session;
mod storage;

#[actix_web::get("/ws")]
async fn ws_endpoint(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    // todo: share the ThreadRng for a bit better performance. But it's a bit tricky since
    //       ThreadRng is !Send and !Sync. Interior mutability might work but that might lead
    //       to some messy code.
    ws::WsResponseBuilder::new(session::Session { id: rand::random() }, &req, stream)
        .protocols(&["dalang"])
        .start()
}

pub async fn start(serve_static: Option<PathBuf>) -> std::io::Result<()> {
    HttpServer::new(move || {
        let mut app = App::new()
            .wrap(middleware::Logger::default())
            .service(ws_endpoint);

        if let Some(static_files) = &serve_static {
            app = app.service(actix_files::Files::new("/", static_files));
        }

        app
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

mod server {
    use std::collections::HashMap;

    use actix::{Actor, Context, Addr};

    use crate::auth::Authenticator;

    use super::storage::Storage;

    pub struct DalangServer<AuthActor: Authenticator> {
        authenticator: Option<Addr<AuthActor>>,
        storages: HashMap<u64, Addr<Storage>>,
    }

    impl<AuthActor: Authenticator> Actor for DalangServer<AuthActor> {
        type Context = Context<Self>;
    }
}