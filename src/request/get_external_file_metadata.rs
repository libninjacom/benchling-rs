use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct GetExternalFileMetadataRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub entry_id: String,
    pub external_file_id: String,
}
impl<'a> GetExternalFileMetadataRequest<'a> {
    pub async fn send(self) -> anyhow::Result<EntryExternalFileById> {
        let mut r = self
            .client
            .client
            .get(
                &format!(
                    "/entries/{entry_id}/external-files/{external_file_id}", entry_id =
                    self.entry_id, external_file_id = self.external_file_id
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
