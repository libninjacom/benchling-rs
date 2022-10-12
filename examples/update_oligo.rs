#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::UpdateOligoRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = UpdateOligoRequired {
        bases: "your bases",
        name: "your name",
        oligo_id: "your oligo id",
        folder_id: "your folder id",
        author_ids: &["your author ids"],
        fields: Fields {},
        schema_id: "your schema id",
        custom_fields: CustomFields {},
        aliases: &["your aliases"],
    };
    let response = client.update_oligo(args).send().await.unwrap();
    println!("{:#?}", response);
}
