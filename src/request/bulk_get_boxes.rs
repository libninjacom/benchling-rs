use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct BulkGetBoxesRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub box_ids: Option<String>,
    pub barcodes: Option<String>,
}
impl<'a> BulkGetBoxesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<BoxesBulkGet> {
        let mut r = self.client.client.get("/boxes:bulk-get");
        if let Some(ref unwrapped) = self.box_ids {
            r = r.push_query("boxIds", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.barcodes {
            r = r.push_query("barcodes", &unwrapped.to_string());
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
    pub fn box_ids(mut self, box_ids: &str) -> Self {
        self.box_ids = Some(box_ids.to_owned());
        self
    }
    pub fn barcodes(mut self, barcodes: &str) -> Self {
        self.barcodes = Some(barcodes.to_owned());
        self
    }
}
