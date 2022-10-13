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
                molecule_base_request : MoleculeBaseRequest { chemical_structure :
                Some(MoleculeStructure { value : Some("your value".to_owned()),
                structure_format : Some(::serde_json::json!({})) }), name :
                Some("your name".to_owned()), author_ids : Some(vec!["your author ids"
                .to_owned()]), folder_id : Some("your folder id".to_owned()), schema_id :
                Some("your schema id".to_owned()), custom_fields : Some(CustomFields {}),
                aliases : Some(vec!["your aliases".to_owned()]), fields : Some(Fields {})
                }, entity_registry_id : "your entity registry id".to_owned() }, id :
                "your id".to_owned() }
            ],
        )
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
