use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetBlobRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub blob_id: String,
}
impl<'a> GetBlobRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Blob> {
        let mut r = self
            .client
            .client
            .get(&format!("/blobs/{blob_id}", blob_id = self.blob_id));
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