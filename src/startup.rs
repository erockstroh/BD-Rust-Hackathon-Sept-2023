use crate::routes::health_check;
use crate::routes::get_users;
use crate::routes::add_user;
use crate::routes::get_price;
use crate::routes::buy_order;
use crate::routes::sell_order;
use actix_web::middleware::Logger;
use actix_web::{
    dev::Server,
    web::{self},
    App, HttpServer,
};
use sqlx::PgPool;
use std::net::TcpListener;

pub fn run(listener: TcpListener, db_connection_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_connection = web::Data::new(db_connection_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/users", web::get().to(get_users))
            .route("/users", web::post().to(add_user))
            .route("/price/{ticker}", web::get().to(get_price))
            .route("/order/buy", web::post().to(buy_order))
            .route("/order/sell", web::post().to(sell_order))
            .app_data(db_connection.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
