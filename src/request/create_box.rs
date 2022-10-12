use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateBoxRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub barcode: Option<String>,
    pub fields: Option<Fields>,
    pub name: Option<String>,
    pub parent_storage_id: Option<String>,
    pub project_id: Option<String>,
    pub schema_id: String,
}
impl<'a> CreateBoxRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Box> {
        let mut r = self.client.client.post("/boxes");
        if let Some(ref unwrapped) = self.barcode {
            r = r.push_json(json!({ "barcode" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.fields {
            r = r.push_json(json!({ "fields" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.name {
            r = r.push_json(json!({ "name" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.parent_storage_id {
            r = r.push_json(json!({ "parentStorageId" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.project_id {
            r = r.push_json(json!({ "projectId" : unwrapped }));
        }
        r = r.push_json(json!({ "schemaId" : self.schema_id }));
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
    pub fn barcode(mut self, barcode: &str) -> Self {
        self.barcode = Some(barcode.to_owned());
        self
    }
    pub fn fields(mut self, fields: Fields) -> Self {
        self.fields = Some(fields);
        self
    }
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }
    pub fn parent_storage_id(mut self, parent_storage_id: &str) -> Self {
        self.parent_storage_id = Some(parent_storage_id.to_owned());
        self
    }
    pub fn project_id(mut self, project_id: &str) -> Self {
        self.project_id = Some(project_id.to_owned());
        self
    }
}
