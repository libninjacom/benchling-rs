#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::UpdateDnaOligoRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = UpdateDnaOligoRequired {
        fields: Fields {},
        folder_id: "your folder id",
        name: "your name",
        bases: "your bases",
        schema_id: "your schema id",
        custom_fields: CustomFields {},
        aliases: &["your aliases"],
        oligo_id: "your oligo id",
        author_ids: &["your author ids"],
    };
    let response = client.update_dna_oligo(args).send().await.unwrap();
    println!("{:#?}", response);
}
