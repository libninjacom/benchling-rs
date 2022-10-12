use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ListWorkflowTaskGroupsRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub ids: Option<String>,
    pub schema_id: Option<String>,
    pub folder_id: Option<String>,
    pub project_id: Option<String>,
    pub mentioned_in: Option<String>,
    pub watcher_ids: Option<String>,
    pub execution_types: Option<String>,
    pub responsible_team_ids: Option<String>,
    pub status_ids_any_of: Option<String>,
    pub status_ids_none_of: Option<String>,
    pub status_ids_only: Option<String>,
    pub name: Option<String>,
    pub name_includes: Option<String>,
    pub creator_ids: Option<String>,
    pub modified_at: Option<String>,
    pub next_token: Option<String>,
    pub page_size: Option<i64>,
    pub display_ids: Option<String>,
    pub archive_reason: Option<String>,
}
impl<'a> ListWorkflowTaskGroupsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<WorkflowTaskGroupsPaginatedList> {
        let mut r = self.client.client.get("/workflow-task-groups");
        if let Some(ref unwrapped) = self.ids {
            r = r.push_query("ids", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.schema_id {
            r = r.push_query("schemaId", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.folder_id {
            r = r.push_query("folderId", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.project_id {
            r = r.push_query("projectId", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.mentioned_in {
            r = r.push_query("mentionedIn", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.watcher_ids {
            r = r.push_query("watcherIds", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.execution_types {
            r = r.push_query("executionTypes", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.responsible_team_ids {
            r = r.push_query("responsibleTeamIds", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.status_ids_any_of {
            r = r.push_query("statusIds.anyOf", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.status_ids_none_of {
            r = r.push_query("statusIds.noneOf", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.status_ids_only {
            r = r.push_query("statusIds.only", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.name {
            r = r.push_query("name", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.name_includes {
            r = r.push_query("nameIncludes", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.creator_ids {
            r = r.push_query("creatorIds", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.modified_at {
            r = r.push_query("modifiedAt", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.next_token {
            r = r.push_query("nextToken", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.page_size {
            r = r.push_query("pageSize", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.display_ids {
            r = r.push_query("displayIds", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.archive_reason {
            r = r.push_query("archiveReason", &unwrapped.to_string());
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
    pub fn ids(mut self, ids: &str) -> Self {
        self.ids = Some(ids.to_owned());
        self
    }
    pub fn schema_id(mut self, schema_id: &str) -> Self {
        self.schema_id = Some(schema_id.to_owned());
        self
    }
    pub fn folder_id(mut self, folder_id: &str) -> Self {
        self.folder_id = Some(folder_id.to_owned());
        self
    }
    pub fn project_id(mut self, project_id: &str) -> Self {
        self.project_id = Some(project_id.to_owned());
        self
    }
    pub fn mentioned_in(mut self, mentioned_in: &str) -> Self {
        self.mentioned_in = Some(mentioned_in.to_owned());
        self
    }
    pub fn watcher_ids(mut self, watcher_ids: &str) -> Self {
        self.watcher_ids = Some(watcher_ids.to_owned());
        self
    }
    pub fn execution_types(mut self, execution_types: &str) -> Self {
        self.execution_types = Some(execution_types.to_owned());
        self
    }
    pub fn responsible_team_ids(mut self, responsible_team_ids: &str) -> Self {
        self.responsible_team_ids = Some(responsible_team_ids.to_owned());
        self
    }
    pub fn status_ids_any_of(mut self, status_ids_any_of: &str) -> Self {
        self.status_ids_any_of = Some(status_ids_any_of.to_owned());
        self
    }
    pub fn status_ids_none_of(mut self, status_ids_none_of: &str) -> Self {
        self.status_ids_none_of = Some(status_ids_none_of.to_owned());
        self
    }
    pub fn status_ids_only(mut self, status_ids_only: &str) -> Self {
        self.status_ids_only = Some(status_ids_only.to_owned());
        self
    }
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }
    pub fn name_includes(mut self, name_includes: &str) -> Self {
        self.name_includes = Some(name_includes.to_owned());
        self
    }
    pub fn creator_ids(mut self, creator_ids: &str) -> Self {
        self.creator_ids = Some(creator_ids.to_owned());
        self
    }
    pub fn modified_at(mut self, modified_at: &str) -> Self {
        self.modified_at = Some(modified_at.to_owned());
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
    pub fn display_ids(mut self, display_ids: &str) -> Self {
        self.display_ids = Some(display_ids.to_owned());
        self
    }
    pub fn archive_reason(mut self, archive_reason: &str) -> Self {
        self.archive_reason = Some(archive_reason.to_owned());
        self
    }
}
