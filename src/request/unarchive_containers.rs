use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UnarchiveContainersRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub container_ids: Vec<String>,
}
impl<'a> UnarchiveContainersRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ContainersArchivalChange> {
        let mut r = self.client.client.post("/containers:unarchive");
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
}
