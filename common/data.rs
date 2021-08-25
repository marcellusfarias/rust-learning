use serde_json::{Map};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Billing {
    pub client_name: String,
    pub products_ordered: HashMap<String, Product>, //product and quantity
}

impl Billing {
    pub fn serialize(&self) -> serde_json::Value {
        json!({
            "client_name": self.client_name,
            "products_ordered": self.products_ordered
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub name: String,
    pub price: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MakeOrderRequest {
    pub client_name: String,
    pub product_name: String
}

impl MakeOrderRequest {
    pub fn serialize(&self) -> serde_json::Value {
        json!({
            "client_name": self.client_name,
            "product_name": self.product_name
        })
    }
}