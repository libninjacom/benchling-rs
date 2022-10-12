#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let container_ids = &["your container ids"];
    let label_template_id = "your label template id";
    let printer_id = "your printer id";
    let response = client
        .print_labels(container_ids, label_template_id, printer_id)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
