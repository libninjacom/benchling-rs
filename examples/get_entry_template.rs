#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let entry_template_id = "your entry template id";
    let response = client.get_entry_template(entry_template_id).send().await.unwrap();
    println!("{:#?}", response);
}
