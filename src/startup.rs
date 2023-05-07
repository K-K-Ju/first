use actix_web::{HttpServer, App, web};
use actix_web::dev::Server;
use env_logger::Env;
use std::net::TcpListener;
use actix_web::middleware::Logger;
use sqlx::PgPool;
use crate::routes::{health_check, subscriptions};

pub fn run(listener: TcpListener, connection_pool: PgPool) -> Result<Server, std::io::Error> {
    env_logger::Builder::from_env(Env::default().default_filter_or("trace")).init();
    let connection_pool = web::Data::new(connection_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(subscriptions::subscribe)
            .service(health_check::health_check)
            .app_data(connection_pool.clone())
    })
        .listen(listener)?
        .run();
    Ok(server)
}