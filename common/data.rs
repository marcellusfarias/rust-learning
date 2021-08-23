use serde_json::{Map};
use serde::{Deserialize, Serialize};


pub struct Billing {
    pub client_name: String,
    pub items: Map<String, f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub name: String,
    pub price: f32,
}