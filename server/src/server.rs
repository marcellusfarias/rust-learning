use actix_web::{web, HttpResponse, Responder};
use std::fs;
use textcode::iso8859_1;

use std::collections::HashMap;

#[path = "../../common/data.rs"]
mod data;
use data::{Product, Billing, MakeOrderRequest};

pub const SERVER_ADDRESS: &str = "127.0.0.1:8080"; 

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
    
    let mut list_products: Vec<Product> = serde_json::from_str::<Vec<Product>>(fs::read_to_string("./data/products.json").unwrap().as_str()).unwrap();
    
    if !list_products.iter().any(|i: &Product| i.name == request.product_name) {
        println!("make_order: error 2");
        return HttpResponse::ExpectationFailed()
            .json(["Product ", request.product_name.as_str(), " doesnt exist"].join(""))
    }
    
    println!("Updating order for the client {} with product {}", request.client_name, request.product_name);

    let mut list_billings: Vec<Billing> = Vec::<Billing>::new();

    match fs::read_to_string("./data/billings.json") {
        Err(e) => {

        }, 
        Ok(billings) => {
            if billings.trim().len() > 0 {
                list_billings = serde_json::from_str(billings.as_str()).unwrap();
            }
        }
    }

    if  list_billings.is_empty() || !list_billings.iter().any(|i: &Billing| i.client_name == request.client_name) {
        let product_index =  list_products.iter().position(|r: &Product| r.name == request.product_name).unwrap();
        let mut map: HashMap<String, Product> = HashMap::<String, Product>::new();
        let product: &Product = &list_products[product_index];
        
        map.insert("1".to_string(), Product {name: product.name.as_str().to_string(), price: product.price});
        // map.insert(vector, 1);

        let new_bill: Billing = Billing {
            client_name: request.client_name,
            products_ordered: map
        };

        println!("Creating billing.json...");

        fs::write("./data/billings.json", new_bill.serialize().to_string());
    }
    else{
        println!("Not creating billing.json...");
    }

    HttpResponse::Ok().body("200")
}