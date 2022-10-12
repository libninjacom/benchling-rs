use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UnarchiveAutomationOutputProcessorsRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub automation_output_processor_ids: Vec<String>,
}
impl<'a> UnarchiveAutomationOutputProcessorsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AutomationOutputProcessorArchivalChange> {
        let mut r = self.client.client.post("/automation-output-processors:unarchive");
        r = r
            .push_json(
                json!(
                    { "automationOutputProcessorIds" : self
                    .automation_output_processor_ids }
                ),
            );
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
