use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct PatchRequestRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub request_id: String,
    pub assignees: Vec<serde_json::Value>,
    pub fields: Fields,
    pub project_id: String,
    pub requestor_id: Option<String>,
    pub sample_groups: Vec<RequestSampleGroupCreate>,
    pub scheduled_on: String,
    pub request_status: String,
}
impl<'a> PatchRequestRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Request> {
        let mut r = self
            .client
            .client
            .patch(&format!("/requests/{request_id}", request_id = self.request_id));
        r = r.push_json(json!({ "assignees" : self.assignees }));
        r = r.push_json(json!({ "fields" : self.fields }));
        r = r.push_json(json!({ "projectId" : self.project_id }));
        if let Some(ref unwrapped) = self.requestor_id {
            r = r.push_json(json!({ "requestorId" : unwrapped }));
        }
        r = r.push_json(json!({ "sampleGroups" : self.sample_groups }));
        r = r.push_json(json!({ "scheduledOn" : self.scheduled_on }));
        r = r.push_json(json!({ "requestStatus" : self.request_status }));
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
    pub fn requestor_id(mut self, requestor_id: &str) -> Self {
        self.requestor_id = Some(requestor_id.to_owned());
        self
    }
}
pub struct PatchRequestRequired<'a> {
    pub request_id: &'a str,
    pub assignees: Vec<serde_json::Value>,
    pub fields: Fields,
    pub project_id: &'a str,
    pub sample_groups: Vec<RequestSampleGroupCreate>,
    pub scheduled_on: &'a str,
    pub request_status: &'a str,
}
impl<'a> PatchRequestRequired<'a> {}
