use actix_web::{middleware, App, HttpServer};
use env_logger::{self, Env};

mod service;
mod ui;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    log::info!("Starting server...");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(service::ping)
            .service(ui::svc::index)
        // Add additional services here
    })
    .bind(("0.0.0.0", 9365))?
    .run()
    .await?;

    log::info!("Server has started and is running");

    Ok(())
}
