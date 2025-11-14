extern crate tokio;
extern crate actix_web;

pub mod configurations;
pub mod routes;
pub mod startup;

//
// #[derive(serde::Deserialize)]
// struct Subscription {
//     email: String,
//     name: String,
// }
//
// async fn greet(req: HttpRequest) -> impl Responder {
//     let name = req.match_info().get("name").unwrap_or("World");
//     format!("Hello, {}!", name)
// }
//

//
// async fn subscribe(_form: Form<Subscription>) -> HttpResponse {
//     HttpResponse::Ok().finish()
// }
