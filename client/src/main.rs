use rdkafka::consumer::{Consumer};

#[path = "../../common/kafka.rs"]
mod kafka;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let consumer = kafka::create_consumer();

    loop {
        match consumer.recv().await {
            Err(e) => {

            }
            Ok(msg) => {

            }
        }
    }

    // get list products
    // display the options
    // post the option
    // get bill
}
