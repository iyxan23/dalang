use actix::{Actor, StreamHandler, Handler, Addr};
use actix_web_actors::ws;

use crate::{server::DalangServer, auth};

/// Represents a WebSocket session
pub struct Session<AuthActor: auth::Authenticator> {
    /// A unique ID
    pub id: usize,
    pub server: Addr<DalangServer<AuthActor>>
}

impl<A: auth::Authenticator> Actor for Session<A> {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("[id:{}] client connected! sending protocol information", self.id);

        let Ok(payload) = dalang_protocol::protocol_version_packet() else {
            println!("[id:{}] failed to run protocol_version_packet(), closing with error", self.id);

            // close when we failed to generate the protocol version packet
            ctx.close(Some(ws::CloseReason { code: ws::CloseCode::Error, description: None }));
            return;
        };

        // as we connect, the server should send its protocol version, with maybe some extensions
        ctx.binary(payload);
    }
}

/// Handler for ws::Message message
impl<AuthActor: auth::Authenticator> StreamHandler<Result<ws::Message, ws::ProtocolError>> for Session<AuthActor> {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),

            Ok(ws::Message::Text(text)) => {
                println!("[id:{}] rececived a text message: `{}`\ndisconnecting", self.id, text); // todo: change to use a logging system

                ctx.close(Some(ws::CloseReason {
                    code: ws::CloseCode::Unsupported,
                    description: Some(String::from("expected a binary message")),
                }));
            },

            Ok(ws::Message::Close(msg)) => {
                println!(
                    "[id:{}] client closed the connection{}",
                    self.id,
                    msg.map(|reason|
                        format!(
                            " with code: `{:?}` and description: `{}`",
                            reason.code,
                            reason.description.unwrap_or(String::new())
                        )
                    ).unwrap_or(String::new())
                );
            }

            Err(err) => {
                println!("[id:{}] websocket protocol error: {:?}", self.id, err);
            }

            Ok(ws::Message::Binary(bin)) => {
                // now its time to process this message
            },

            _ => (),
        }
    }
}

impl<A: auth::Authenticator> Handler<messages::RawMessage> for Session<A> {
    type Result = ();

    fn handle(&mut self, msg: messages::RawMessage, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            messages::RawMessage::Text(text) => ctx.text(text),
            messages::RawMessage::Binary(bytes) => ctx.binary(bytes),
        }
    }
}

pub mod messages {
    use actix::Message;

    // Send a raw message directly, do not use except really needed
    pub enum RawMessage {
        Text(String),
        Binary(Vec<u8>)
    }

    impl Message for RawMessage {
        type Result = ();
    }
}