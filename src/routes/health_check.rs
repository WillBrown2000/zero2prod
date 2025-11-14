use actix_web::HttpResponse;

// Make the handler public so it can be used by the startup module
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}