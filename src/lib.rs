use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::web::Form;
use actix_web::dev::Server;
use std::net::TcpListener;


#[derive(serde::Deserialize)]
struct Subscription {
    email: String,
    name: String,
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello, {}!", name)
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

async fn subscribe(_form: Form<Subscription>) -> HttpResponse {
    HttpResponse::Ok().finish()
}



pub fn run(tcp_listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new( || {
        App::new()
            .route("/subscriptions", web::post().to(subscribe))
            .route("/", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))

    })
        .listen(tcp_listener)?
        .run();
    Ok(server)
}