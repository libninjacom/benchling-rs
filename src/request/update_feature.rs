use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UpdateFeatureRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub feature_id: String,
}
impl<'a> UpdateFeatureRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Feature> {
        let mut r = self
            .client
            .client
            .patch(&format!("/features/{feature_id}", feature_id = self.feature_id));
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
