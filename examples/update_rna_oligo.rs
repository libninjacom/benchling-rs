#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::UpdateRnaOligoRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = UpdateRnaOligoRequired {
        author_ids: &["your author ids"],
        name: "your name",
        folder_id: "your folder id",
        oligo_id: "your oligo id",
        bases: "your bases",
        fields: Fields {},
        custom_fields: CustomFields {},
        aliases: &["your aliases"],
        schema_id: "your schema id",
    };
    let response = client.update_rna_oligo(args).send().await.unwrap();
    println!("{:#?}", response);
}
