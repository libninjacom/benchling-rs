use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateCustomEntityRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub aliases: Vec<String>,
    pub author_ids: Vec<String>,
    pub custom_fields: CustomFields,
    pub fields: Fields,
    pub folder_id: String,
    pub name: String,
    pub schema_id: String,
    pub entity_registry_id: String,
    pub naming_strategy: String,
    pub registry_id: String,
}
impl<'a> CreateCustomEntityRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CustomEntity> {
        let mut r = self.client.client.post("/custom-entities");
        r = r.push_json(json!({ "aliases" : self.aliases }));
        r = r.push_json(json!({ "authorIds" : self.author_ids }));
        r = r.push_json(json!({ "customFields" : self.custom_fields }));
        r = r.push_json(json!({ "fields" : self.fields }));
        r = r.push_json(json!({ "folderId" : self.folder_id }));
        r = r.push_json(json!({ "name" : self.name }));
        r = r.push_json(json!({ "schemaId" : self.schema_id }));
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
pub struct CreateCustomEntityRequired<'a> {
    pub aliases: &'a [&'a str],
    pub author_ids: &'a [&'a str],
    pub custom_fields: CustomFields,
    pub fields: Fields,
    pub folder_id: &'a str,
    pub name: &'a str,
    pub schema_id: &'a str,
    pub entity_registry_id: &'a str,
    pub naming_strategy: &'a str,
    pub registry_id: &'a str,
}
impl<'a> CreateCustomEntityRequired<'a> {}
