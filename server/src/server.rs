use serde_json::{Map, Value};
use actix_web::{get, post, HttpResponse, Responder};
use std::fs;

#[path = "../../common/data.rs"]
mod data;
use data::Product;

pub const SERVER_ADDRESS: &str = "127.0.0.1:8080"; 

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

pub async fn list_products() -> impl Responder {
    let list_products = serde_json::from_str::<Vec<Product>>(fs::read_to_string("./data/products.json").unwrap().as_str()).unwrap();
    HttpResponse::Ok().json(list_products)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}