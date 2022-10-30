#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::UpdateRnaOligoRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = UpdateRnaOligoRequired {
        aliases: &["your aliases"],
        fields: Fields {},
        folder_id: "your folder id",
        author_ids: &["your author ids"],
        name: "your name",
        custom_fields: CustomFields {},
        bases: "your bases",
        schema_id: "your schema id",
        oligo_id: "your oligo id",
    };
    let response = client.update_rna_oligo(args).send().await.unwrap();
    println!("{:#?}", response);
}
