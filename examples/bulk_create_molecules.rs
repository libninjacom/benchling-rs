#![allow(unused_imports)]
use benchling::BenchlingClient;
use benchling::model::*;
#[tokio::main]
async fn main() {
    let client = BenchlingClient::from_env();
    let response = client
        .bulk_create_molecules()
        .molecules(
            vec![
                MoleculeCreate { create_entity_into_registry : CreateEntityIntoRegistry {
                naming_strategy : Some("your naming strategy".to_owned()),
                entity_registry_id : Some("your entity registry id".to_owned()),
                folder_id : Some("your folder id".to_owned()), registry_id :
                Some("your registry id".to_owned()) }, molecule_base_request_for_create :
                MoleculeBaseRequestForCreate { molecule_base_request :
                MoleculeBaseRequest { schema_id : Some("your schema id".to_owned()),
                aliases : Some(vec!["your aliases".to_owned()]), custom_fields :
                Some(CustomFields {}), chemical_structure : Some(MoleculeStructure {
                value : Some("your value".to_owned()), structure_format :
                Some(::serde_json::json!({})) }), folder_id : Some("your folder id"
                .to_owned()), name : Some("your name".to_owned()), fields : Some(Fields
                {}), author_ids : Some(vec!["your author ids".to_owned()]) } } }
            ],
        )
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
