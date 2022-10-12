use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateBlobRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub data64: String,
    pub md5: String,
    pub mime_type: Option<String>,
    pub name: String,
    pub type_: String,
}
impl<'a> CreateBlobRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Blob> {
        let mut r = self.client.client.post("/blobs");
        r = r.push_json(json!({ "data64" : self.data64 }));
        r = r.push_json(json!({ "md5" : self.md5 }));
        if let Some(ref unwrapped) = self.mime_type {
            r = r.push_json(json!({ "mimeType" : unwrapped }));
        }
        r = r.push_json(json!({ "name" : self.name }));
        r = r.push_json(json!({ "type" : self.type_ }));
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
    pub fn mime_type(mut self, mime_type: &str) -> Self {
        self.mime_type = Some(mime_type.to_owned());
        self
    }
}
pub struct CreateBlobRequired<'a> {
    pub data64: &'a str,
    pub md5: &'a str,
    pub name: &'a str,
    pub type_: &'a str,
}
impl<'a> CreateBlobRequired<'a> {}
