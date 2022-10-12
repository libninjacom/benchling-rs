use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ListAutomationInputGeneratorsRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub assay_run_id: String,
    pub next_token: Option<String>,
    pub modified_at: Option<String>,
}
impl<'a> ListAutomationInputGeneratorsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AutomationFileInputsPaginatedList> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/assay-runs/{assay_run_id}/automation-input-generators",
                    assay_run_id = self.assay_run_id
                ),
            );
        if let Some(ref unwrapped) = self.next_token {
            r = r.push_query("nextToken", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.modified_at {
            r = r.push_query("modifiedAt", &unwrapped.to_string());
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
    pub fn next_token(mut self, next_token: &str) -> Self {
        self.next_token = Some(next_token.to_owned());
        self
    }
    pub fn modified_at(mut self, modified_at: &str) -> Self {
        self.modified_at = Some(modified_at.to_owned());
        self
    }
}
