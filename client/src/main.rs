
#[path = "../../common/data.rs"]
mod data;

use std::io::{self, BufRead};

use reqwest::Response;

#[tokio::main]
async fn main() {
    println!("Welcome to Rust Restaurant!");
    println!("Please, tell me your name:");
    
    let stdin = io::stdin();
    let client_name = stdin.lock()
        .lines()
        .next()
        .expect("there was no next line")
        .expect("the line could not be read");

    let mut list_products: Vec<data::Product>;
    let client = reqwest::Client::new();

    let resp = client
        .get("http://127.0.0.1:8080")
        .json("/")
        .send()
        .await
        .map_err(|e| println!("{:?}", e));

    println!("{:?}", resp.unwrap().text().await.unwrap());

    // if !resp.status().is_success() {
    //     return Err(AppError::HttpBadStatus(resp.status()));
    // } else {
    //     resp.bytes().await.map_err(|e| AppError::ReqwestError(e))
    // }

    // loop {
    //     println!("It's a pleasure to have you here, {}!", client_name);
    //     println!("Let's show you what we have for lunch today:");
    //     // show_interface();
    // }

    // get list products
    // display the options
    // post the option
    // get bill
}
