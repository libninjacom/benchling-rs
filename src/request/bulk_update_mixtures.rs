use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct BulkUpdateMixturesRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub mixtures: Option<Vec<MixtureBulkUpdate>>,
}
impl<'a> BulkUpdateMixturesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AsyncTaskLink> {
        let mut r = self.client.client.post("/mixtures:bulk-update");
        if let Some(ref unwrapped) = self.mixtures {
            r = r.push_json(json!({ "mixtures" : unwrapped }));
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
    pub fn mixtures(mut self, mixtures: Vec<MixtureBulkUpdate>) -> Self {
        self.mixtures = Some(mixtures);
        self
    }
}
