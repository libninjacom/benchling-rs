#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::UpdateOligoRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = UpdateOligoRequired {
        aliases: &["your aliases"],
        fields: Fields {},
        folder_id: "your folder id",
        author_ids: &["your author ids"],
        schema_id: "your schema id",
        bases: "your bases",
        custom_fields: CustomFields {},
        name: "your name",
        oligo_id: "your oligo id",
    };
    let response = client.update_oligo(args).send().await.unwrap();
    println!("{:#?}", response);
}
