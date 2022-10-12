use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct BulkGetBlobsRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub blob_ids: Option<String>,
}
impl<'a> BulkGetBlobsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<BlobsBulkGet> {
        let mut r = self.client.client.get("/blobs:bulk-get");
        if let Some(ref unwrapped) = self.blob_ids {
            r = r.push_query("blobIds", &unwrapped.to_string());
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
    pub fn blob_ids(mut self, blob_ids: &str) -> Self {
        self.blob_ids = Some(blob_ids.to_owned());
        self
    }
}
