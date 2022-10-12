use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ExecuteRequestsSampleGroupsRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub request_id: String,
    pub sample_groups: Vec<SampleGroupStatusUpdate>,
}
impl<'a> ExecuteRequestsSampleGroupsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ExecuteSampleGroups> {
        let mut r = self
            .client
            .client
            .post(
                &format!(
                    "/requests/{request_id}:execute-sample-groups", request_id = self
                    .request_id
                ),
            );
        r = r.push_json(json!({ "sampleGroups" : self.sample_groups }));
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
