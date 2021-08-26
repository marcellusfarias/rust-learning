use actix_web::{web, HttpResponse, Responder};
use std::fs;
use textcode::iso8859_1;

#[path = "../../common/data.rs"]
mod data;
use data::{Billing, MakeOrderRequest, Product};

pub const SERVER_ADDRESS: &str = "127.0.0.1:8080";

pub async fn list_products() -> impl Responder {
    let list_products = data::get_list_products();
    HttpResponse::Ok().json(list_products)
}

pub async fn make_order(raw_request: web::Bytes) -> impl Responder {
    let raw_req = iso8859_1::decode_to_string(&raw_request.to_vec());
    let request: MakeOrderRequest = data::get_make_order_request(raw_req.as_str());
    let list_products: Vec<Product> = data::get_list_products();

    if !list_products
        .iter()
        .any(|i: &Product| i.name == request.product_name)
    {
        return HttpResponse::ExpectationFailed()
            .json(["Product ", request.product_name.as_str(), " doesnt exist"].join(""));
    }

    println!(
        "Updating order for the client {} with product {}",
        request.client_name, request.product_name
    );

    let mut list_billings: Vec<Billing> = Vec::<Billing>::new();

    match fs::read_to_string("./data/billings.json") {
        Err(e) => {}
        Ok(billings) => {
            if billings.trim().len() > 0 {
                list_billings = serde_json::from_str::<Vec<Billing>>(billings.as_str()).unwrap();
            }
        }
    }

    let product_index = list_products
        .iter()
        .position(|r: &Product| r.name == request.product_name)
        .unwrap();
    let product: &Product = &list_products[product_index];

    if list_billings.is_empty()
        || !list_billings
            .iter()
            .any(|i: &Billing| i.client_name == request.client_name)
    {
        let mut products: Vec<Product> = Vec::<Product>::new();
        products.push(product.clone());

        list_billings.push(Billing {
            client_name: request.client_name,
            products: products,
        });

        fs::write(
            "./data/billings.json",
            serde_json::json!(list_billings).to_string(),
        );
    } else {
        let mut bill = list_billings
            .iter()
            .filter(|&x| *x.client_name == request.client_name)
            .nth(0)
            .unwrap()
            .clone();

        bill.products.push(product.clone());

        list_billings = list_billings
            .iter()
            .filter(|&x| *x.client_name != request.client_name)
            .cloned()
            .collect();

        list_billings.push(bill);

        fs::write(
            "./data/billings.json",
            serde_json::json!(list_billings).to_string(),
        );
    }
    
    println!("Bill updated!");
    HttpResponse::Ok().body("200")
}
