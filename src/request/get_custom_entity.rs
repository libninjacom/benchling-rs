use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetCustomEntityRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub custom_entity_id: String,
}
impl<'a> GetCustomEntityRequest<'a> {
    pub async fn send(self) -> anyhow::Result<CustomEntity> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/custom-entities/{custom_entity_id}", custom_entity_id = self
                    .custom_entity_id
                ),
            );
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
