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
                MoleculeCreate { molecule_base_request_for_create :
                MoleculeBaseRequestForCreate { molecule_base_request :
                MoleculeBaseRequest { aliases : Some(vec!["your aliases".to_owned()]),
                author_ids : Some(vec!["your author ids".to_owned()]), custom_fields :
                Some(CustomFields {}), fields : Some(Fields {}), folder_id :
                Some("your folder id".to_owned()), chemical_structure :
                Some(MoleculeStructure { structure_format :
                Some(::serde_json::json!({})), value : Some("your value".to_owned()) }),
                name : Some("your name".to_owned()), schema_id : Some("your schema id"
                .to_owned()) } }, create_entity_into_registry : CreateEntityIntoRegistry
                { entity_registry_id : Some("your entity registry id".to_owned()),
                naming_strategy : Some("your naming strategy".to_owned()), registry_id :
                Some("your registry id".to_owned()), folder_id : Some("your folder id"
                .to_owned()) } }
            ],
        )
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
