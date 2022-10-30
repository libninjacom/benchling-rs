#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::UpdateOligoRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = UpdateOligoRequired {
        folder_id: "your folder id",
        oligo_id: "your oligo id",
        name: "your name",
        schema_id: "your schema id",
        author_ids: &["your author ids"],
        fields: Fields {},
        bases: "your bases",
        custom_fields: CustomFields {},
        aliases: &["your aliases"],
    };
    let response = client.update_oligo(args).send().await.unwrap();
    println!("{:#?}", response);
}
