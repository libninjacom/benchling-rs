use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CompleteMultipartBlobRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub blob_id: String,
    pub parts: Option<Vec<BlobPart>>,
}
impl<'a> CompleteMultipartBlobRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Blob> {
        let mut r = self
            .client
            .client
            .post(&format!("/blobs/{blob_id}:complete-upload", blob_id = self.blob_id));
        if let Some(ref unwrapped) = self.parts {
            r = r.push_json(json!({ "parts" : unwrapped }));
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
    pub fn parts(mut self, parts: Vec<BlobPart>) -> Self {
        self.parts = Some(parts);
        self
    }
}
