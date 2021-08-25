use actix_web::{HttpServer, App, web, Responder, HttpResponse};
mod server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server...");

    HttpServer::new(|| {
        App::new()
            .route("/order", web::post().to(server::make_order))
            .route("/list_products", web::get().to(server::list_products))
    }).bind(server::SERVER_ADDRESS)?
      .run()
      .await
}
