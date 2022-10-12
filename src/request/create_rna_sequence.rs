use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateRnaSequenceRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub aliases: Vec<String>,
    pub annotations: Vec<RnaAnnotation>,
    pub author_ids: Vec<String>,
    pub bases: String,
    pub custom_fields: CustomFields,
    pub fields: Fields,
    pub folder_id: String,
    pub is_circular: bool,
    pub name: String,
    pub primers: Vec<Primer>,
    pub schema_id: String,
    pub translations: Vec<Translation>,
    pub entity_registry_id: String,
    pub naming_strategy: String,
    pub registry_id: String,
}
impl<'a> CreateRnaSequenceRequest<'a> {
    pub async fn send(self) -> anyhow::Result<RnaSequence> {
        let mut r = self.client.client.post("/rna-sequences");
        r = r.push_json(json!({ "aliases" : self.aliases }));
        r = r.push_json(json!({ "annotations" : self.annotations }));
        r = r.push_json(json!({ "authorIds" : self.author_ids }));
        r = r.push_json(json!({ "bases" : self.bases }));
        r = r.push_json(json!({ "customFields" : self.custom_fields }));
        r = r.push_json(json!({ "fields" : self.fields }));
        r = r.push_json(json!({ "folderId" : self.folder_id }));
        r = r.push_json(json!({ "isCircular" : self.is_circular }));
        r = r.push_json(json!({ "name" : self.name }));
        r = r.push_json(json!({ "primers" : self.primers }));
        r = r.push_json(json!({ "schemaId" : self.schema_id }));
        r = r.push_json(json!({ "translations" : self.translations }));
        r = r.push_json(json!({ "entityRegistryId" : self.entity_registry_id }));
        r = r.push_json(json!({ "namingStrategy" : self.naming_strategy }));
        r = r.push_json(json!({ "registryId" : self.registry_id }));
        r = self.client.authenticate(r);
        let res = r.send().await.unwrap().error_for_status();
        match res {
            Ok(res) => res.json().await.map_err(|e| anyhow::anyhow!("{:?}", e)),
            Err(res) => {
                let text = res.text().await.map_err(|e| anyhow::anyhow!("{:?}", e))?;
                Err(anyhow::anyhow!("{:?}", text))
            }
        }
    }
}
pub struct CreateRnaSequenceRequired<'a> {
    pub aliases: &'a [&'a str],
    pub annotations: Vec<RnaAnnotation>,
    pub author_ids: &'a [&'a str],
    pub bases: &'a str,
    pub custom_fields: CustomFields,
    pub fields: Fields,
    pub folder_id: &'a str,
    pub is_circular: bool,
    pub name: &'a str,
    pub primers: Vec<Primer>,
    pub schema_id: &'a str,
    pub translations: Vec<Translation>,
    pub entity_registry_id: &'a str,
    pub naming_strategy: &'a str,
    pub registry_id: &'a str,
}
impl<'a> CreateRnaSequenceRequired<'a> {}
