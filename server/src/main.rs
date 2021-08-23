use serde_json::{Map, Value};
use std::fs;



#[path = "../../common/data.rs"]
mod data;
use data::Product;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    let contents = fs::read_to_string("./data/products.json")
        .expect("Something went wrong reading the file");

    match serde_json::from_str::<Vec<Product>>(contents.as_str())  {
        Err(e) => {
            println!("Unable to convert JSON to Struct. Content: {:#?}. Error is {:?}", contents.as_str(), e);
        }
        Ok(vector) => {
            for x in vector {
                println!("{:?}", &x);
            }
        }
    }

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
