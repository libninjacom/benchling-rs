#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let input_generator_id = "your input generator id";
    let response = client
        .generate_input_with_automation_input_generator(input_generator_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
