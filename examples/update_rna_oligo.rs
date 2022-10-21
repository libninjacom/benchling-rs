#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::UpdateRnaOligoRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = UpdateRnaOligoRequired {
        bases: "your bases",
        schema_id: "your schema id",
        oligo_id: "your oligo id",
        fields: Fields {},
        author_ids: &["your author ids"],
        folder_id: "your folder id",
        name: "your name",
        custom_fields: CustomFields {},
        aliases: &["your aliases"],
    };
    let response = client.update_rna_oligo(args).send().await.unwrap();
    println!("{:#?}", response);
}
