use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ListProjectsRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub next_token: Option<String>,
    pub page_size: Option<i64>,
    pub sort: Option<String>,
    pub archive_reason: Option<String>,
    pub ids: Option<String>,
    pub name: Option<String>,
}
impl<'a> ListProjectsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<ProjectsPaginatedList> {
        let mut r = self.client.client.get("/projects");
        if let Some(ref unwrapped) = self.next_token {
            r = r.push_query("nextToken", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.page_size {
            r = r.push_query("pageSize", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.sort {
            r = r.push_query("sort", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.archive_reason {
            r = r.push_query("archiveReason", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.ids {
            r = r.push_query("ids", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.name {
            r = r.push_query("name", &unwrapped.to_string());
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
    pub fn next_token(mut self, next_token: &str) -> Self {
        self.next_token = Some(next_token.to_owned());
        self
    }
    pub fn page_size(mut self, page_size: i64) -> Self {
        self.page_size = Some(page_size);
        self
    }
    pub fn sort(mut self, sort: &str) -> Self {
        self.sort = Some(sort.to_owned());
        self
    }
    pub fn archive_reason(mut self, archive_reason: &str) -> Self {
        self.archive_reason = Some(archive_reason.to_owned());
        self
    }
    pub fn ids(mut self, ids: &str) -> Self {
        self.ids = Some(ids.to_owned());
        self
    }
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }
}
