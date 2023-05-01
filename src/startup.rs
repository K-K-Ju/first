use actix_web::{HttpServer, App};
use actix_web::dev::Server;
use std::net::TcpListener;
use crate::routes::{health_check, subscriptions};

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .service(subscriptions::subscribe)
            .service(health_check::health_check)
    })
        .listen(listener)?
        .run();
    Ok(server)
}