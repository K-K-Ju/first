use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

//#[actix_web::main] // or
#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(greet)
            .service(health_check)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
