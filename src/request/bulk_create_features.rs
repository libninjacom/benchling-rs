use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct BulkCreateFeaturesRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub features: Option<Vec<FeatureBulkCreate>>,
}
impl<'a> BulkCreateFeaturesRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AsyncTaskLink> {
        let mut r = self.client.client.post("/features:bulk-create");
        if let Some(ref unwrapped) = self.features {
            r = r.push_json(json!({ "features" : unwrapped }));
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
    pub fn features(mut self, features: Vec<FeatureBulkCreate>) -> Self {
        self.features = Some(features);
        self
    }
}
