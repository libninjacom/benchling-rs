#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::UpdateRnaOligoRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = UpdateRnaOligoRequired {
        custom_fields: CustomFields {},
        fields: Fields {},
        schema_id: "your schema id",
        folder_id: "your folder id",
        aliases: &["your aliases"],
        oligo_id: "your oligo id",
        author_ids: &["your author ids"],
        bases: "your bases",
        name: "your name",
    };
    let response = client.update_rna_oligo(args).send().await.unwrap();
    println!("{:#?}", response);
}
