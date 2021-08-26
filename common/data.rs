use serde_json::{Map};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::fs;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Billing {
    pub client_name: String,
    pub products: Vec<Product>,
}

impl Billing {
    pub fn serialize(&self) -> serde_json::Value {
        json!({
            "client_name": self.client_name,
            "products": self.products
        })
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    pub name: String,
    pub price: f32,
}

pub fn get_list_products() -> Vec<Product> {
    serde_json::from_str::<Vec<Product>>(fs::read_to_string("./data/products.json").unwrap().as_str()).unwrap()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MakeOrderRequest {
    pub client_name: String,
    pub product_name: String
}

pub fn get_make_order_request(request: &str) -> MakeOrderRequest {
    serde_json::from_str::<MakeOrderRequest>(request).unwrap()
}

impl MakeOrderRequest {
    pub fn serialize(&self) -> serde_json::Value {
        json!({
            "client_name": self.client_name,
            "product_name": self.product_name
        })
    }
}