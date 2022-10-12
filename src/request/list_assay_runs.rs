use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ListAssayRunsRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub schema_id: String,
    pub min_created_time: Option<i64>,
    pub max_created_time: Option<i64>,
    pub next_token: Option<String>,
    pub page_size: Option<i64>,
    pub ids: Option<String>,
}
impl<'a> ListAssayRunsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AssayRunsPaginatedList> {
        let mut r = self.client.client.get("/assay-runs");
        r = r.push_query("schemaId", &self.schema_id.to_string());
        if let Some(ref unwrapped) = self.min_created_time {
            r = r.push_query("minCreatedTime", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.max_created_time {
            r = r.push_query("maxCreatedTime", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.next_token {
            r = r.push_query("nextToken", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.page_size {
            r = r.push_query("pageSize", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ids {
            r = r.push_query("ids", &unwrapped.to_string());
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
    pub fn min_created_time(mut self, min_created_time: i64) -> Self {
        self.min_created_time = Some(min_created_time);
        self
    }
    pub fn max_created_time(mut self, max_created_time: i64) -> Self {
        self.max_created_time = Some(max_created_time);
        self
    }
    pub fn next_token(mut self, next_token: &str) -> Self {
        self.next_token = Some(next_token.to_owned());
        self
    }
    pub fn page_size(mut self, page_size: i64) -> Self {
        self.page_size = Some(page_size);
        self
    }
    pub fn ids(mut self, ids: &str) -> Self {
        self.ids = Some(ids.to_owned());
        self
    }
}
