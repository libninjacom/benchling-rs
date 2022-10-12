use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct BulkGetBatchesRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub batch_ids: Option<String>,
    pub batch_names: Option<String>,
    pub registry_id: Option<String>,
}
impl<'a> BulkGetBatchesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<BatchesBulkGet> {
        let mut r = self.client.client.get("/batches:bulk-get");
        if let Some(ref unwrapped) = self.batch_ids {
            r = r.push_query("batchIds", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.batch_names {
            r = r.push_query("batchNames", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.registry_id {
            r = r.push_query("registryId", &unwrapped.to_string());
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
    pub fn batch_ids(mut self, batch_ids: &str) -> Self {
        self.batch_ids = Some(batch_ids.to_owned());
        self
    }
    pub fn batch_names(mut self, batch_names: &str) -> Self {
        self.batch_names = Some(batch_names.to_owned());
        self
    }
    pub fn registry_id(mut self, registry_id: &str) -> Self {
        self.registry_id = Some(registry_id.to_owned());
        self
    }
}
