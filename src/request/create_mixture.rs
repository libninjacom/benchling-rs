use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateMixtureRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub aliases: Vec<String>,
    pub amount: String,
    pub author_ids: Vec<String>,
    pub custom_fields: CustomFields,
    pub entity_registry_id: String,
    pub fields: Fields,
    pub folder_id: String,
    pub ingredients: Vec<IngredientWriteParams>,
    pub name: String,
    pub schema_id: String,
    pub units: Option<String>,
    pub naming_strategy: String,
    pub registry_id: String,
}
impl<'a> CreateMixtureRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Mixture> {
        let mut r = self.client.client.post("/mixtures");
        r = r.push_json(json!({ "aliases" : self.aliases }));
        r = r.push_json(json!({ "amount" : self.amount }));
        r = r.push_json(json!({ "authorIds" : self.author_ids }));
        r = r.push_json(json!({ "customFields" : self.custom_fields }));
        r = r.push_json(json!({ "entityRegistryId" : self.entity_registry_id }));
        r = r.push_json(json!({ "fields" : self.fields }));
        r = r.push_json(json!({ "folderId" : self.folder_id }));
        r = r.push_json(json!({ "ingredients" : self.ingredients }));
        r = r.push_json(json!({ "name" : self.name }));
        r = r.push_json(json!({ "schemaId" : self.schema_id }));
        if let Some(ref unwrapped) = self.units {
            r = r.push_json(json!({ "units" : unwrapped }));
        }
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
    pub fn units(mut self, units: &str) -> Self {
        self.units = Some(units.to_owned());
        self
    }
}
pub struct CreateMixtureRequired<'a> {
    pub aliases: &'a [&'a str],
    pub amount: &'a str,
    pub author_ids: &'a [&'a str],
    pub custom_fields: CustomFields,
    pub entity_registry_id: &'a str,
    pub fields: Fields,
    pub folder_id: &'a str,
    pub ingredients: Vec<IngredientWriteParams>,
    pub name: &'a str,
    pub schema_id: &'a str,
    pub naming_strategy: &'a str,
    pub registry_id: &'a str,
}
impl<'a> CreateMixtureRequired<'a> {}
