use serde::{Deserialize, Serialize};
use serde_json::json;
mod data;
use data::Product;

#[derive(Serialize, Deserialize, Debug)]
pub struct MakeOrderRequest {
    pub client_name: String,
    pub product_name: String
}

impl MakeOrderRequest {
    pub fn make_order(&self) -> serde_json::Value {
        json!({
            "client_name": self.client_name,
            "product_name": self.product_name
        })
    }
}