use actix_web::{dev::Server, web, App, HttpServer};
use std::net::TcpListener;

use crate::routes::{health_check, subscribe};

pub fn run(tcp_listener: TcpListener) -> Result<Server, std::io::Error> {
    let server: Server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(tcp_listener)?
    .run();

    Ok(server)
}
