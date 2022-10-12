use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct BulkGetPlatesRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub plate_ids: Option<String>,
    pub barcodes: Option<String>,
    pub returning: Option<String>,
}
impl<'a> BulkGetPlatesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<PlatesBulkGet> {
        let mut r = self.client.client.get("/plates:bulk-get");
        if let Some(ref unwrapped) = self.plate_ids {
            r = r.push_query("plateIds", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.barcodes {
            r = r.push_query("barcodes", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.returning {
            r = r.push_query("returning", &unwrapped.to_string());
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
    pub fn plate_ids(mut self, plate_ids: &str) -> Self {
        self.plate_ids = Some(plate_ids.to_owned());
        self
    }
    pub fn barcodes(mut self, barcodes: &str) -> Self {
        self.barcodes = Some(barcodes.to_owned());
        self
    }
    pub fn returning(mut self, returning: &str) -> Self {
        self.returning = Some(returning.to_owned());
        self
    }
}
