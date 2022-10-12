use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UpdateEntryRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub entry_id: String,
    pub author_ids: Option<String>,
    pub fields: Option<Fields>,
    pub folder_id: Option<String>,
    pub name: Option<String>,
    pub schema_id: Option<String>,
}
impl<'a> UpdateEntryRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Entry> {
        let mut r = self
            .client
            .client
            .patch(&format!("/entries/{entry_id}", entry_id = self.entry_id));
        if let Some(ref unwrapped) = self.author_ids {
            r = r.push_json(json!({ "authorIds" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.fields {
            r = r.push_json(json!({ "fields" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.folder_id {
            r = r.push_json(json!({ "folderId" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.name {
            r = r.push_json(json!({ "name" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.schema_id {
            r = r.push_json(json!({ "schemaId" : unwrapped }));
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
    pub fn author_ids(mut self, author_ids: &str) -> Self {
        self.author_ids = Some(author_ids.to_owned());
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
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }
    pub fn schema_id(mut self, schema_id: &str) -> Self {
        self.schema_id = Some(schema_id.to_owned());
        self
    }
}
