use actix::{Actor, StreamHandler};
use actix_web_actors::ws;

/// Represents a WebSocket session
pub struct Session {
    /// A unique ID
    pub id: usize,
}

impl Actor for Session {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Session {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                ctx.pong(&msg)
            },
            Ok(ws::Message::Text(text)) => {
                ctx.text(text)
            },
            Ok(ws::Message::Binary(bin)) => {
                ctx.binary(bin)
            },
            _ => (),
        }
    }
}
