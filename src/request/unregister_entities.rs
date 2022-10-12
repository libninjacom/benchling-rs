use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UnregisterEntitiesRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub registry_id: String,
    pub entity_ids: Vec<String>,
    pub folder_id: String,
}
impl<'a> UnregisterEntitiesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<EmptyObject> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/registries/{registry_id}:unregister-entities", registry_id = self
                    .registry_id
                ),
            );
        r = r.push_json(json!({ "entityIds" : self.entity_ids }));
        r = r.push_json(json!({ "folderId" : self.folder_id }));
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
