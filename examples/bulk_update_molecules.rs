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
                MoleculeBulkUpdate(::serde_json::json!({}),
                MoleculeUpdate(::serde_json::json!({}), MoleculeBaseRequest { schema_id :
                Some("your schema id".to_owned()), author_ids :
                Some(vec!["your author ids".to_owned()]), fields : Some(Fields {}),
                folder_id : Some("your folder id".to_owned()), aliases :
                Some(vec!["your aliases".to_owned()]), name : Some("your name"
                .to_owned()), chemical_structure : Some(MoleculeStructure { value :
                Some("your value".to_owned()), structure_format :
                Some(::serde_json::json!({})) }), custom_fields : Some(CustomFields {})
                }))
            ],
        )
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
