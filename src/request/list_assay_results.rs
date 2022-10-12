use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ListAssayResultsRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub schema_id: String,
    pub created_at_lt: Option<String>,
    pub created_at_gt: Option<String>,
    pub created_at_lte: Option<String>,
    pub created_at_gte: Option<String>,
    pub min_created_time: Option<i64>,
    pub max_created_time: Option<i64>,
    pub sort: Option<String>,
    pub next_token: Option<String>,
    pub page_size: Option<i64>,
    pub entity_ids: Option<String>,
    pub storage_ids: Option<String>,
    pub assay_run_ids: Option<String>,
    pub ids: Option<String>,
}
impl<'a> ListAssayResultsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<AssayResultsPaginatedList> {
        let mut r = self.client.client.get("/assay-results");
        r = r.push_query("schemaId", &self.schema_id.to_string());
        if let Some(ref unwrapped) = self.created_at_lt {
            r = r.push_query("createdAt.lt", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.created_at_gt {
            r = r.push_query("createdAt.gt", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.created_at_lte {
            r = r.push_query("createdAt.lte", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.created_at_gte {
            r = r.push_query("createdAt.gte", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.min_created_time {
            r = r.push_query("minCreatedTime", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.max_created_time {
            r = r.push_query("maxCreatedTime", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.sort {
            r = r.push_query("sort", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.next_token {
            r = r.push_query("nextToken", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.page_size {
            r = r.push_query("pageSize", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.entity_ids {
            r = r.push_query("entityIds", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.storage_ids {
            r = r.push_query("storageIds", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.assay_run_ids {
            r = r.push_query("assayRunIds", &unwrapped.to_string());
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
    pub fn created_at_lt(mut self, created_at_lt: &str) -> Self {
        self.created_at_lt = Some(created_at_lt.to_owned());
        self
    }
    pub fn created_at_gt(mut self, created_at_gt: &str) -> Self {
        self.created_at_gt = Some(created_at_gt.to_owned());
        self
    }
    pub fn created_at_lte(mut self, created_at_lte: &str) -> Self {
        self.created_at_lte = Some(created_at_lte.to_owned());
        self
    }
    pub fn created_at_gte(mut self, created_at_gte: &str) -> Self {
        self.created_at_gte = Some(created_at_gte.to_owned());
        self
    }
    pub fn min_created_time(mut self, min_created_time: i64) -> Self {
        self.min_created_time = Some(min_created_time);
        self
    }
    pub fn max_created_time(mut self, max_created_time: i64) -> Self {
        self.max_created_time = Some(max_created_time);
        self
    }
    pub fn sort(mut self, sort: &str) -> Self {
        self.sort = Some(sort.to_owned());
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
    pub fn entity_ids(mut self, entity_ids: &str) -> Self {
        self.entity_ids = Some(entity_ids.to_owned());
        self
    }
    pub fn storage_ids(mut self, storage_ids: &str) -> Self {
        self.storage_ids = Some(storage_ids.to_owned());
        self
    }
    pub fn assay_run_ids(mut self, assay_run_ids: &str) -> Self {
        self.assay_run_ids = Some(assay_run_ids.to_owned());
        self
    }
    pub fn ids(mut self, ids: &str) -> Self {
        self.ids = Some(ids.to_owned());
        self
    }
}
