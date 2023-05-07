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
    let request_id = Uuid::new_v4();
    let request_span = tracing::info_span!(
        "Adding new subscriber:",
        %request_id,
        subscriber_name=%form.name,
        subscriber_email=%form.email);

    let _request_span_guard = request_span.enter();

    tracing::info!("{} - Saving subscriber data in the database...", &request_id);
    match sqlx::query!(r#"INSERT INTO subscriptions (id, email, name, subscribed_at)
            VALUES ($1, $2, $3, $4);"#,
            Uuid::new_v4(),
            form.email,
            form.name,
            Utc::now())
        .execute(connection_pool.as_ref())
        .await
    {
        Ok(_) => {
            tracing::info!("{} - New subscriber added", &request_id);
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            tracing::error!("{} - Failed to add new subscriber: {:?}", &request_id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}