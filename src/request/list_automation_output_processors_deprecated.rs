use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ListAutomationOutputProcessorsDeprecatedRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub assay_run_id: String,
    pub next_token: Option<String>,
}
impl<'a> ListAutomationOutputProcessorsDeprecatedRequest<'a> {
    pub async fn send(
        self,
    ) -> anyhow::Result<DeprecatedAutomationOutputProcessorsPaginatedList> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/assay-runs/{assay_run_id}/automation-output-processors",
                    assay_run_id = self.assay_run_id
                ),
            );
        if let Some(ref unwrapped) = self.next_token {
            r = r.push_query("nextToken", &unwrapped.to_string());
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
}
