use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetContainerContentRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub container_id: String,
    pub containable_id: String,
}
impl<'a> GetContainerContentRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ContainerContent> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/containers/{container_id}/contents/{containable_id}", container_id
                    = self.container_id, containable_id = self.containable_id
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
