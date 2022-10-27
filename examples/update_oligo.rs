#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::UpdateOligoRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = UpdateOligoRequired {
        custom_fields: CustomFields {},
        folder_id: "your folder id",
        oligo_id: "your oligo id",
        name: "your name",
        aliases: &["your aliases"],
        bases: "your bases",
        schema_id: "your schema id",
        author_ids: &["your author ids"],
        fields: Fields {},
    };
    let response = client.update_oligo(args).send().await.unwrap();
    println!("{:#?}", response);
}
