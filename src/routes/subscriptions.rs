use actix_web::{HttpResponse, web, post};
use sqlx::PgPool;
use chrono::Utc;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct Subscriber {
    name: String,
    email: String
}

#[post("/subscribe")]
pub async fn subscribe(
    form: web::Form<Subscriber>,
    connection_pool: web::Data<PgPool>) -> HttpResponse
{
    match sqlx::query!(r#"INSERT INTO subscriptions (id, email, name, subscribed_at)
            VALUES ($1, $2, $3, $4);"#,
            Uuid::new_v4(),
            form.email,
            form.name,
            Utc::now())
        .execute(connection_pool.as_ref())
        .await
    {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}