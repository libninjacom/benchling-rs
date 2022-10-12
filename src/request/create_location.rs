use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateLocationRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub barcode: Option<String>,
    pub fields: Option<Fields>,
    pub name: String,
    pub parent_storage_id: Option<String>,
    pub schema_id: String,
}
impl<'a> CreateLocationRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Location> {
        let mut r = self.client.client.post("/locations");
        if let Some(ref unwrapped) = self.barcode {
            r = r.push_json(json!({ "barcode" : unwrapped }));
        }
        if let Some(ref unwrapped) = self.fields {
            r = r.push_json(json!({ "fields" : unwrapped }));
        }
        r = r.push_json(json!({ "name" : self.name }));
        if let Some(ref unwrapped) = self.parent_storage_id {
            r = r.push_json(json!({ "parentStorageId" : unwrapped }));
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
    pub fn parent_storage_id(mut self, parent_storage_id: &str) -> Self {
        self.parent_storage_id = Some(parent_storage_id.to_owned());
        self
    }
}
