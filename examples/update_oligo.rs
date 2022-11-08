#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::UpdateOligoRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = UpdateOligoRequired {
        oligo_id: "your oligo id",
        author_ids: &["your author ids"],
        bases: "your bases",
        fields: Fields {},
        custom_fields: CustomFields {},
        aliases: &["your aliases"],
        name: "your name",
        folder_id: "your folder id",
        schema_id: "your schema id",
    };
    let response = client.update_oligo(args).send().await.unwrap();
    println!("{:#?}", response);
}
