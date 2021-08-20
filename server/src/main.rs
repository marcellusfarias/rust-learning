use serde_json::{Map, Value};
use std::fs;



#[path = "../../common/data.rs"]
mod data;
use data::Product;

fn main() {
    println!("Hello, world!");

    let contents = fs::read_to_string("./data/products.json")
        .expect("Something went wrong reading the file");

    match serde_json::from_str::<Vec<Product>>(contents.as_str())  {
        Err(e) => {
            println!("Unable to convert JSON to Struct. Message received from topic is: {:#?}. Error is {:?}", contents.as_str(), e);
        }
        Ok(vector) => {
            for x in vector {
                println!("{:?}", &x);
            }
        }
    }
}
