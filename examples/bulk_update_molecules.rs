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
                MoleculeBulkUpdate { id : "your id".to_owned(), molecule_update :
                MoleculeUpdate { molecule_base_request : MoleculeBaseRequest { aliases :
                Some(vec!["your aliases".to_owned()]), name : Some("your name"
                .to_owned()), fields : Some(Fields {}), schema_id : Some("your schema id"
                .to_owned()), author_ids : Some(vec!["your author ids".to_owned()]),
                chemical_structure : Some(MoleculeStructure { structure_format :
                Some(::serde_json::json!({})), value : Some("your value".to_owned()) }),
                custom_fields : Some(CustomFields {}), folder_id : Some("your folder id"
                .to_owned()) }, entity_registry_id : "your entity registry id".to_owned()
                } }
            ],
        )
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
