use std::time::SystemTime;

use actix_web::{web, HttpResponse};
use sqlx::{
    types::chrono::{DateTime, Utc},
    PgPool,
};

use crate::models::Order;
use crate::routes::get_ticker_price;
use uuid::Uuid;

pub async fn buy_order(form: web::Json<Order>, db_pool: web::Data<PgPool>) -> HttpResponse {
    log::info!("Buy! Buy! Buy! √");

    // fetch current order price
    let (quote, error) = get_ticker_price(&form.ticker).await;

    if error != "" {
        return HttpResponse::BadRequest().json(error);
    }

    let order_type: &str;

    if form.limit_order {
        if quote > form.limit_price {
            let error = format!(
                "Order price higher than limit price. Difference: {}",
                form.limit_price - quote
            );

            return HttpResponse::BadRequest().json(error);
        }

        order_type = "Limit Buy";
    } else {
        order_type = "Market Buy";
    }

    let id = Uuid::new_v4();
    let date: DateTime<Utc> = SystemTime::now().clone().into();

    let total_amount = form.quantity as f64 * quote as f64;

    let results = sqlx::query!(
        r#"INSERT INTO transaction_history (
        id,
        user_id,
        asset_ticker,
        price,
        quantity,
        total_amount,
        purchased_at,
        order_type
    )
    VALUES
        ($1, $2, $3, $4, $5, $6, $7, $8);"#,
        id as sqlx::types::Uuid,
        form.user_id.clone() as String,
        form.ticker.clone() as String,
        quote as f64,
        form.quantity.clone() as i64,
        total_amount as f64,
        date as DateTime<Utc>,
        order_type as &str
    )
    .execute(&**db_pool)
    .await;

    if results.is_ok() {
        HttpResponse::Created().json(total_amount)
    } else {
        let error = results.unwrap_err().to_string();

        HttpResponse::InternalServerError().body(error)
    }
}

pub async fn sell_order(form: web::Json<Order>, db_pool: web::Data<PgPool>) -> HttpResponse {
    log::info!("Sell! Sell! Sell! √");

    // fetch current order price
    let (quote, error) = get_ticker_price(&form.ticker).await;

    if error != "" {
        return HttpResponse::BadRequest().json(error);
    }

    let order_type: &str;

    if form.limit_order {
        if quote < form.limit_price {
            let error = format!(
                "Order price lower than limit price. Difference: {}",
                quote - form.limit_price
            );

            return HttpResponse::BadRequest().json(error);
        }

        order_type = "Limit Sell";
    } else {
        order_type = "Market Sell";
    }

    let id = Uuid::new_v4();
    let date: DateTime<Utc> = SystemTime::now().clone().into();

    let total_amount = form.quantity as f64 * quote as f64;

    let results = sqlx::query!(
        r#"INSERT INTO transaction_history (
        id,
        user_id,
        asset_ticker,
        price,
        quantity,
        total_amount,
        purchased_at,
        order_type
    )
    VALUES
        ($1, $2, $3, $4, $5, $6, $7, $8);"#,
        id as sqlx::types::Uuid,
        form.user_id.clone() as String,
        form.ticker.clone() as String,
        quote as f64,
        form.quantity.clone() as i64,
        total_amount as f64,
        date as DateTime<Utc>,
        order_type as &str
    )
    .execute(&**db_pool)
    .await;

    if results.is_ok() {
        HttpResponse::Created().json(total_amount)
    } else {
        let error = results.unwrap_err().to_string();

        HttpResponse::InternalServerError().body(error)
    }
}
