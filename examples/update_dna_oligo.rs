#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::UpdateDnaOligoRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = UpdateDnaOligoRequired {
        aliases: &["your aliases"],
        folder_id: "your folder id",
        name: "your name",
        oligo_id: "your oligo id",
        custom_fields: CustomFields {},
        schema_id: "your schema id",
        bases: "your bases",
        author_ids: &["your author ids"],
        fields: Fields {},
    };
    let response = client.update_dna_oligo(args).send().await.unwrap();
    println!("{:#?}", response);
}
