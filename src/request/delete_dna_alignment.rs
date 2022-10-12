use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct DeleteDnaAlignmentRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub dna_alignment_id: String,
}
impl<'a> DeleteDnaAlignmentRequest<'a> {
    pub async fn send(self) -> anyhow::Result<EmptyObject> {
        let mut r = self
            .client
            .client
            .delete(
                &format!(
                    "/dna-alignments/{dna_alignment_id}", dna_alignment_id = self
                    .dna_alignment_id
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
