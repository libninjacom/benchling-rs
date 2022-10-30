#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .bulk_update_molecules()
        .molecules(
            vec![
                MoleculeBulkUpdate { molecule_update : MoleculeUpdate {
                entity_registry_id : "your entity registry id".to_owned(),
                molecule_base_request : MoleculeBaseRequest { schema_id :
                Some("your schema id".to_owned()), aliases : Some(vec!["your aliases"
                .to_owned()]), custom_fields : Some(CustomFields {}), chemical_structure
                : Some(MoleculeStructure { value : Some("your value".to_owned()),
                structure_format : Some(::serde_json::json!({})) }), folder_id :
                Some("your folder id".to_owned()), name : Some("your name".to_owned()),
                fields : Some(Fields {}), author_ids : Some(vec!["your author ids"
                .to_owned()]) } }, id : "your id".to_owned() }
            ],
        )
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
