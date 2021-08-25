use serde_json::{Map, Value};
use actix_web::{get, post, web, HttpResponse, Responder};
use std::fs;
use textcode::iso8859_1;

#[path = "../../common/data.rs"]
mod data;
use data::Product;

#[path = "../../common/types.rs"]
mod types;
use types::MakeOrderRequest;

pub const SERVER_ADDRESS: &str = "127.0.0.1:8080"; 

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub async fn list_products() -> impl Responder {
    let list_products = serde_json::from_str::<Vec<Product>>(fs::read_to_string("./data/products.json").unwrap().as_str()).unwrap();
    HttpResponse::Ok().json(list_products)
}

pub async fn make_order(raw_request: web::Bytes) -> impl Responder {
    let raw_req = iso8859_1::decode_to_string(&raw_request.to_vec());    
    let request: MakeOrderRequest = match serde_json::from_str(raw_req.as_str()) {
        Ok(val) => val,
        Err(_) => {
            println!("make_order: error 1");
            println!("request: {}", raw_req.as_str());
            return HttpResponse::ExpectationFailed()
                .json("Service was unable to deserialize the request")
        }
    };
    
    let list_products: Vec<Product> = serde_json::from_str::<Vec<Product>>(fs::read_to_string("./data/products.json").unwrap().as_str()).unwrap();
    
    if !list_products.iter().any(|i: &Product| i.name == request.product_name) {
        println!("make_order: error 2");
        return HttpResponse::ExpectationFailed()
            .json(["Product ", request.product_name.as_str(), " doesnt exist"].join(""))
    }
    
    println!("Updating order for the client {} with product {}", request.client_name, request.product_name);

    HttpResponse::Ok().body("200")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}