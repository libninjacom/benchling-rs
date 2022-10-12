use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ReserveContainersRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub assignee_id: String,
    pub comment: Option<String>,
    pub container_ids: Vec<String>,
}
impl<'a> ReserveContainersRequest<'a> {
    pub async fn send(self) -> anyhow::Result<EmptyObject> {
        let mut r = self.client.client.post("/containers:reserve");
        r = r.push_json(json!({ "assigneeId" : self.assignee_id }));
        if let Some(ref unwrapped) = self.comment {
            r = r.push_json(json!({ "comment" : unwrapped }));
        }
        r = r.push_json(json!({ "containerIds" : self.container_ids }));
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
    pub fn comment(mut self, comment: &str) -> Self {
        self.comment = Some(comment.to_owned());
        self
    }
}
