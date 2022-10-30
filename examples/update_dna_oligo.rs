#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::UpdateDnaOligoRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = UpdateDnaOligoRequired {
        custom_fields: CustomFields {},
        bases: "your bases",
        oligo_id: "your oligo id",
        name: "your name",
        folder_id: "your folder id",
        schema_id: "your schema id",
        fields: Fields {},
        author_ids: &["your author ids"],
        aliases: &["your aliases"],
    };
    let response = client.update_dna_oligo(args).send().await.unwrap();
    println!("{:#?}", response);
}
