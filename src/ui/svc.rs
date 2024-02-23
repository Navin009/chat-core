use actix_web::{get, HttpResponse, Responder};
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    title: String,
    message: String,
}

#[get("/")]
pub async fn index() -> impl Responder {
    let template = IndexTemplate {
        title: "Welcome".to_string(),
        message: "Hello, world!".to_string(),
    };

    HttpResponse::Ok().body(template.render().unwrap())
}
