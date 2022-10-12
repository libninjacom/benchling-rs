use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateEntryRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub author_ids: Option<serde_json::Value>,
    pub custom_fields: Option<CustomFields>,
    pub entry_template_id: Option<String>,
    pub fields: Option<Fields>,
    pub folder_id: String,
    pub initial_tables: Option<Vec<InitialTable>>,
    pub name: String,
    pub schema_id: Option<String>,
}
impl<'a> CreateEntryRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Entry> {
        let mut r = self.client.client.post("/entries");
        if let Some(ref unwrapped) = self.author_ids {
            r = r.push_json(json!({ "authorIds" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.custom_fields {
            r = r.push_json(json!({ "customFields" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.entry_template_id {
            r = r.push_json(json!({ "entryTemplateId" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.fields {
            r = r.push_json(json!({ "fields" : unwrapped }));
        }
        r = r.push_json(json!({ "folderId" : self.folder_id }));
        if let Some(ref unwrapped) = self.initial_tables {
            r = r.push_json(json!({ "initialTables" : unwrapped }));
        }
        r = r.push_json(json!({ "name" : self.name }));
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
    pub fn author_ids(mut self, author_ids: serde_json::Value) -> Self {
        self.author_ids = Some(author_ids);
        self
    }
    pub fn custom_fields(mut self, custom_fields: CustomFields) -> Self {
        self.custom_fields = Some(custom_fields);
        self
    }
    pub fn entry_template_id(mut self, entry_template_id: &str) -> Self {
        self.entry_template_id = Some(entry_template_id.to_owned());
        self
    }
    pub fn fields(mut self, fields: Fields) -> Self {
        self.fields = Some(fields);
        self
    }
    pub fn initial_tables(mut self, initial_tables: Vec<InitialTable>) -> Self {
        self.initial_tables = Some(initial_tables);
        self
    }
    pub fn schema_id(mut self, schema_id: &str) -> Self {
        self.schema_id = Some(schema_id.to_owned());
        self
    }
}
