use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetEntryTemplateRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub entry_template_id: String,
}
impl<'a> GetEntryTemplateRequest<'a> {
    pub async fn send(self) -> anyhow::Result<EntryTemplate> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/entry-templates/{entry_template_id}", entry_template_id = self
                    .entry_template_id
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
