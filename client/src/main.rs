
#[path = "../../common/data.rs"]
mod data;

use std::io::{self, BufRead};

use reqwest::Response;

const server_address: &str = "http://127.0.0.1:8080";

#[tokio::main]
async fn main() {
    println!("Welcome to Rust Restaurant!");
    println!("Please, tell me your name:");
    
    let stdin = io::stdin();
    let stdout = std::io::stdout();
    let client_name = stdin.lock()
        .lines()
        .next()
        .expect("there was no next line")
        .expect("the line could not be read");
   
    let client = reqwest::Client::new();
    let resp = client
        .get([server_address, "/list_products"].join(""))
        .send()
        .await
        .map_err(|e| println!("{:?}", e));

    let mut  list_products = serde_json::from_str::<Vec<data::Product>>(resp.unwrap().text().await.unwrap().as_str()).unwrap();

    // loop {
        println!("It's a pleasure to have you here, {}!", client_name);
        println!("Let's show you what we have for lunch today:");
        
        //list vector
        for (i, x) in list_products.iter().enumerate() {
            println!("{} - {:?}", i + 1, x);
        }

        // need to learn how to do a pause before putting on a loop
        // let _ = std::process::Command::new("pause").status();
    // }

    // get list products
    // display the options
    // post the option
    // get bill
}
