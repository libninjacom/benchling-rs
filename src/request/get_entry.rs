use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetEntryRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub entry_id: String,
}
impl<'a> GetEntryRequest<'a> {
    pub async fn send(self) -> anyhow::Result<EntryById> {
        let mut r = self
            .client
            .client
            .get(&format!("/entries/{entry_id}", entry_id = self.entry_id));
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
