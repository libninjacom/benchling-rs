use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetBatchRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub batch_id: String,
}
impl<'a> GetBatchRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Batch> {
        let mut r = self
            .client
            .client
            .get(&format!("/batches/{batch_id}", batch_id = self.batch_id));
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
