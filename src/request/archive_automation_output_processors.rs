use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ArchiveAutomationOutputProcessorsRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub automation_output_processor_ids: Vec<String>,
    pub reason: Option<String>,
}
impl<'a> ArchiveAutomationOutputProcessorsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AutomationOutputProcessorArchivalChange> {
        let mut r = self.client.client.post("/automation-output-processors:archive");
        r = r
            .push_json(
                json!(
                    { "automationOutputProcessorIds" : self
                    .automation_output_processor_ids }
                ),
            );
        if let Some(ref unwrapped) = self.reason {
            r = r.push_json(json!({ "reason" : unwrapped }));
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
    pub fn reason(mut self, reason: &str) -> Self {
        self.reason = Some(reason.to_owned());
        self
    }
}
