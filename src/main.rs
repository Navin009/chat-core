use actix_web::{HttpServer, App};

mod service;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(service::ping)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}