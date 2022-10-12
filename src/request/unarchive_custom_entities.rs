use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UnarchiveCustomEntitiesRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub custom_entity_ids: Vec<String>,
}
impl<'a> UnarchiveCustomEntitiesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CustomEntitiesArchivalChange> {
        let mut r = self.client.client.post("/custom-entities:unarchive");
        r = r.push_json(json!({ "customEntityIds" : self.custom_entity_ids }));
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
