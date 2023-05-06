use actix_web::{HttpServer, App, web};
use actix_web::dev::Server;
use std::net::TcpListener;
use sqlx::PgPool;
use crate::routes::{health_check, subscriptions};

pub fn run(listener: TcpListener, connection_pool: PgPool) -> Result<Server, std::io::Error> {
    let connection_pool = web::Data::new(connection_pool);
    let server = HttpServer::new(move || {
        App::new()
            .service(subscriptions::subscribe)
            .service(health_check::health_check)
            .app_data(connection_pool.clone())
    })
        .listen(listener)?
        .run();
    Ok(server)
}