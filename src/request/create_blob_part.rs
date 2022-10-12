use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateBlobPartRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub blob_id: String,
    pub data64: String,
    pub md5: String,
    pub part_number: i64,
}
impl<'a> CreateBlobPartRequest<'a> {
    pub async fn send(self) -> anyhow::Result<BlobPart> {
        let mut r = self
            .client
            .client
            .post(&format!("/blobs/{blob_id}/parts", blob_id = self.blob_id));
        r = r.push_json(json!({ "data64" : self.data64 }));
        r = r.push_json(json!({ "md5" : self.md5 }));
        r = r.push_json(json!({ "partNumber" : self.part_number }));
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
pub struct CreateBlobPartRequired<'a> {
    pub blob_id: &'a str,
    pub data64: &'a str,
    pub md5: &'a str,
    pub part_number: i64,
}
impl<'a> CreateBlobPartRequired<'a> {}
