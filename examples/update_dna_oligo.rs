#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::UpdateDnaOligoRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = UpdateDnaOligoRequired {
        custom_fields: CustomFields {},
        folder_id: "your folder id",
        schema_id: "your schema id",
        aliases: &["your aliases"],
        author_ids: &["your author ids"],
        fields: Fields {},
        bases: "your bases",
        name: "your name",
        oligo_id: "your oligo id",
    };
    let response = client.update_dna_oligo(args).send().await.unwrap();
    println!("{:#?}", response);
}
