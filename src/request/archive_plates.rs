use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ArchivePlatesRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub plate_ids: Vec<String>,
    pub reason: String,
    pub should_remove_barcodes: bool,
}
impl<'a> ArchivePlatesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PlatesArchivalChange> {
        let mut r = self.client.client.post("/plates:archive");
        r = r.push_json(json!({ "plateIds" : self.plate_ids }));
        r = r.push_json(json!({ "reason" : self.reason }));
        r = r.push_json(json!({ "shouldRemoveBarcodes" : self.should_remove_barcodes }));
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
