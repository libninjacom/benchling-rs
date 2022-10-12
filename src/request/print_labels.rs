use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PrintLabelsRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub container_ids: Vec<String>,
    pub label_template_id: String,
    pub printer_id: String,
}
impl<'a> PrintLabelsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<EmptyObject> {
        let mut r = self.client.client.post("/containers:print-labels");
        r = r.push_json(json!({ "containerIds" : self.container_ids }));
        r = r.push_json(json!({ "labelTemplateId" : self.label_template_id }));
        r = r.push_json(json!({ "printerId" : self.printer_id }));
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
