#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
use benchling::request::UpdateMoleculeRequired;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let args = UpdateMoleculeRequired {
        aliases: &["your aliases"],
        molecule_id: "your molecule id",
        folder_id: "your folder id",
        schema_id: "your schema id",
        custom_fields: CustomFields {},
        entity_registry_id: "your entity registry id",
        chemical_structure: MoleculeStructure {
            structure_format: Some(::serde_json::json!({})),
            value: Some("your value".to_owned()),
        },
        name: "your name",
        fields: Fields {},
        author_ids: &["your author ids"],
    };
    let response = client.update_molecule(args).send().await.unwrap();
    println!("{:#?}", response);
}
