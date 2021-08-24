use actix_web::{HttpServer, App, web, Responder, HttpResponse};
mod server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server...");

    HttpServer::new(|| {
        App::new()
            .service(server::hello)
            .service(server::echo)
            .route("/list_products", web::get().to(server::list_products))
    }).bind(server::SERVER_ADDRESS)?
      .run()
      .await
}
