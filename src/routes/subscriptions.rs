use actix_web::{HttpResponse, Responder, web, post};

#[derive(serde::Deserialize)]
pub struct Subscriber {
    name: String,
    email: String
}

#[post("/subscribe")]
pub async fn subscribe(form: web::Form<Subscriber>) -> impl Responder {
    println!("name: {}, email: {}", form.name, form.email);
    HttpResponse::Ok().finish()
}