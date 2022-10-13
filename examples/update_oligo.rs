#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::UpdateOligoRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = UpdateOligoRequired {
        schema_id: "your schema id",
        name: "your name",
        oligo_id: "your oligo id",
        aliases: &["your aliases"],
        fields: Fields {},
        author_ids: &["your author ids"],
        folder_id: "your folder id",
        bases: "your bases",
        custom_fields: CustomFields {},
    };
    let response = client.update_oligo(args).send().await.unwrap();
    println!("{:#?}", response);
}
