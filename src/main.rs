

use std::io;
use std::time;
use actix::prelude::*;
use actix::{Actor, StreamHandler};
use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder, middleware};
use actix_web_actors::ws;

const HEARTBEAT_INTERVAL: time::Duration = time::Duration::from_secs(5);
const SESSION_TIMEOUT: time::Duration = time::Duration::from_secs(10);

struct WsSession {
    // Heartbeat Time
    hb_time: time::Instant,
}

impl WsSession {
    fn new() -> Self {
        Self {
            hb_time: time::Instant::now(),
        }
    }

    fn hb_start(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |actor, ctx| {
            if time::Instant::now().duration_since(actor.hb_time) > SESSION_TIMEOUT {
                ctx.stop();
            } else {
                ctx.ping(b"");
            }
        });
    }
}

impl Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb_start(ctx);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb_time = time::Instant::now();
                ctx.pong(&msg)
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb_time = time::Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                ctx.text(format!("text message from ws: {}", text));
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

#[get("/ws")]
async fn on_ws(
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, actix_http::error::Error> {
    ws::start(WsSession::new(), &req, stream)
}

#[get("/")]
async fn index() -> impl Responder {
    "Helloooo."
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(index)
            .service(on_ws)
            .service(actix_files::Files::new("/webclient/", "webclient/"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
