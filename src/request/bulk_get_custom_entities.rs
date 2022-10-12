use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct BulkGetCustomEntitiesRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub custom_entity_ids: String,
}
impl<'a> BulkGetCustomEntitiesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CustomEntitiesList> {
        let mut r = self.client.client.get("/custom-entities:bulk-get");
        r = r.push_query("customEntityIds", &self.custom_entity_ids.to_string());
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
