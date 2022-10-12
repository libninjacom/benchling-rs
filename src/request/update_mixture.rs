use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UpdateMixtureRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub mixture_id: String,
    pub aliases: Option<Vec<String>>,
    pub amount: Option<String>,
    pub author_ids: Option<Vec<String>>,
    pub custom_fields: Option<CustomFields>,
    pub entity_registry_id: Option<String>,
    pub fields: Option<Fields>,
    pub folder_id: Option<String>,
    pub ingredients: Option<Vec<IngredientWriteParams>>,
    pub name: Option<String>,
    pub schema_id: Option<String>,
    pub units: Option<String>,
}
impl<'a> UpdateMixtureRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Mixture> {
        let mut r = self
            .client
            .client
            .patch(&format!("/mixtures/{mixture_id}", mixture_id = self.mixture_id));
        if let Some(ref unwrapped) = self.aliases {
            r = r.push_json(json!({ "aliases" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.amount {
            r = r.push_json(json!({ "amount" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.author_ids {
            r = r.push_json(json!({ "authorIds" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.custom_fields {
            r = r.push_json(json!({ "customFields" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.entity_registry_id {
            r = r.push_json(json!({ "entityRegistryId" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.fields {
            r = r.push_json(json!({ "fields" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.folder_id {
            r = r.push_json(json!({ "folderId" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.ingredients {
            r = r.push_json(json!({ "ingredients" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.name {
            r = r.push_json(json!({ "name" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.schema_id {
            r = r.push_json(json!({ "schemaId" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.units {
            r = r.push_json(json!({ "units" : unwrapped }));
        }
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
    pub fn aliases(
        mut self,
        aliases: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .aliases = Some(
            aliases.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn amount(mut self, amount: &str) -> Self {
        self.amount = Some(amount.to_owned());
        self
    }
    pub fn author_ids(
        mut self,
        author_ids: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Self {
        self
            .author_ids = Some(
            author_ids.into_iter().map(|s| s.as_ref().to_owned()).collect(),
        );
        self
    }
    pub fn custom_fields(mut self, custom_fields: CustomFields) -> Self {
        self.custom_fields = Some(custom_fields);
        self
    }
    pub fn entity_registry_id(mut self, entity_registry_id: &str) -> Self {
        self.entity_registry_id = Some(entity_registry_id.to_owned());
        self
    }
    pub fn fields(mut self, fields: Fields) -> Self {
        self.fields = Some(fields);
        self
    }
    pub fn folder_id(mut self, folder_id: &str) -> Self {
        self.folder_id = Some(folder_id.to_owned());
        self
    }
    pub fn ingredients(mut self, ingredients: Vec<IngredientWriteParams>) -> Self {
        self.ingredients = Some(ingredients);
        self
    }
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }
    pub fn schema_id(mut self, schema_id: &str) -> Self {
        self.schema_id = Some(schema_id.to_owned());
        self
    }
    pub fn units(mut self, units: &str) -> Self {
        self.units = Some(units.to_owned());
        self
    }
}
