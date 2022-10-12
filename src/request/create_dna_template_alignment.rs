use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateDnaTemplateAlignmentRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub algorithm: String,
    pub files: Vec<serde_json::Value>,
    pub name: String,
    pub template_sequence_id: String,
}
impl<'a> CreateDnaTemplateAlignmentRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AsyncTaskLink> {
        let mut r = self.client.client.post("/dna-alignments:create-template-alignment");
        r = r.push_json(json!({ "algorithm" : self.algorithm }));
        r = r.push_json(json!({ "files" : self.files }));
        r = r.push_json(json!({ "name" : self.name }));
        r = r.push_json(json!({ "templateSequenceId" : self.template_sequence_id }));
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
pub struct CreateDnaTemplateAlignmentRequired<'a> {
    pub algorithm: &'a str,
    pub files: Vec<serde_json::Value>,
    pub name: &'a str,
    pub template_sequence_id: &'a str,
}
impl<'a> CreateDnaTemplateAlignmentRequired<'a> {}
