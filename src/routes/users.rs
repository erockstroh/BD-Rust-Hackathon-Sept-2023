use std::time::SystemTime;

use actix_web::{HttpResponse, web};
use sqlx::{PgPool, types::chrono::{Utc, DateTime}};

use crate::models::User;
use uuid::Uuid;

pub async fn get_users(db_pool: web::Data<PgPool>) ->HttpResponse {
    log::info!("These are not the druids you're looking for √");

    let results = sqlx::query_as!(
        User,
        "SELECT user_id, email, name
        FROM users",
      )
      .fetch_all(&**db_pool)
      .await;

    let users = results.unwrap();

      if !users.is_empty() {
        HttpResponse::Ok().json(users)
      } else {
        HttpResponse::NoContent().finish()
      }
}

pub async fn add_user(form: web::Json<User>, db_pool: web::Data<PgPool>) -> HttpResponse {
    log::info!("Nothing to see here, move along √");

    let id = Uuid::new_v4();
    let date: DateTime<Utc> = SystemTime::now().clone().into();

    let results = sqlx::query!(r#"INSERT INTO users (
        id,
        email,
        name,
        user_id,
        subscribed_at
    )
    VALUES
        ($1, $2, $3, $4, $5);"#,
        id as sqlx::types::Uuid,
        form.email.clone() as String,
        form.name.clone() as String,
        form.user_id.clone() as String,
        date as DateTime<Utc>)
        .execute(&**db_pool)
        .await;

    if results.is_ok() {
        HttpResponse::Created().finish()
    } else {
        let error = results.unwrap_err().to_string();

        HttpResponse::InternalServerError().body(error)
    }
}
