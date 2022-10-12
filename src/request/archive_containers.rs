use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ArchiveContainersRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub container_ids: Vec<String>,
    pub reason: String,
    pub should_remove_barcodes: Option<bool>,
}
impl<'a> ArchiveContainersRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ContainersArchivalChange> {
        let mut r = self.client.client.post("/containers:archive");
        r = r.push_json(json!({ "containerIds" : self.container_ids }));
        r = r.push_json(json!({ "reason" : self.reason }));
        if let Some(ref unwrapped) = self.should_remove_barcodes {
            r = r.push_json(json!({ "shouldRemoveBarcodes" : unwrapped }));
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
    pub fn should_remove_barcodes(mut self, should_remove_barcodes: bool) -> Self {
        self.should_remove_barcodes = Some(should_remove_barcodes);
        self
    }
}
