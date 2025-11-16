use crate::routes::{health_check, subscribe};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;
use sqlx::PgPool;

pub fn run(tcp_listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/subscriptions", web::post().to(subscribe))
            .route("/health_check", web::get().to(health_check))
            // Provide the existing Data<PgPool> to the app; avoid wrapping Data inside Data
            .app_data(pool.clone())
    })
    .listen(tcp_listener)?
    .run();
    Ok(server)
}
