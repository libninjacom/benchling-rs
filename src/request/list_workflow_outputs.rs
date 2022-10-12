use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ListWorkflowOutputsRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub ids: Option<String>,
    pub workflow_task_group_ids: Option<String>,
    pub workflow_task_ids: Option<String>,
    pub schema_id: Option<String>,
    pub watcher_ids: Option<String>,
    pub responsible_team_ids: Option<String>,
    pub creation_origin_ids: Option<String>,
    pub linked_item_ids_any_of: Option<String>,
    pub linked_item_ids_all_of: Option<String>,
    pub linked_item_ids_none_of: Option<String>,
    pub schema_fields: Option<SchemaFieldsQueryParam>,
    pub name: Option<String>,
    pub name_includes: Option<String>,
    pub creator_ids: Option<String>,
    pub modified_at: Option<String>,
    pub next_token: Option<String>,
    pub page_size: Option<i64>,
    pub display_ids: Option<String>,
    pub archive_reason: Option<String>,
}
impl<'a> ListWorkflowOutputsRequest<'a> {
    pub async fn send(self) -> anyhow::Result<WorkflowOutputsPaginatedList> {
        let mut r = self.client.client.get("/workflow-outputs");
        if let Some(ref unwrapped) = self.ids {
            r = r.push_query("ids", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.workflow_task_group_ids {
            r = r.push_query("workflowTaskGroupIds", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.workflow_task_ids {
            r = r.push_query("workflowTaskIds", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.schema_id {
            r = r.push_query("schemaId", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.watcher_ids {
            r = r.push_query("watcherIds", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.responsible_team_ids {
            r = r.push_query("responsibleTeamIds", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.creation_origin_ids {
            r = r.push_query("creationOriginIds", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.linked_item_ids_any_of {
            r = r.push_query("linkedItemIds.anyOf", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.linked_item_ids_all_of {
            r = r.push_query("linkedItemIds.allOf", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.linked_item_ids_none_of {
            r = r.push_query("linkedItemIds.noneOf", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.schema_fields {
            r = r.push_query("schemaFields", &unwrapped.to_string());
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
    pub fn workflow_task_group_ids(mut self, workflow_task_group_ids: &str) -> Self {
        self.workflow_task_group_ids = Some(workflow_task_group_ids.to_owned());
        self
    }
    pub fn workflow_task_ids(mut self, workflow_task_ids: &str) -> Self {
        self.workflow_task_ids = Some(workflow_task_ids.to_owned());
        self
    }
    pub fn schema_id(mut self, schema_id: &str) -> Self {
        self.schema_id = Some(schema_id.to_owned());
        self
    }
    pub fn watcher_ids(mut self, watcher_ids: &str) -> Self {
        self.watcher_ids = Some(watcher_ids.to_owned());
        self
    }
    pub fn responsible_team_ids(mut self, responsible_team_ids: &str) -> Self {
        self.responsible_team_ids = Some(responsible_team_ids.to_owned());
        self
    }
    pub fn creation_origin_ids(mut self, creation_origin_ids: &str) -> Self {
        self.creation_origin_ids = Some(creation_origin_ids.to_owned());
        self
    }
    pub fn linked_item_ids_any_of(mut self, linked_item_ids_any_of: &str) -> Self {
        self.linked_item_ids_any_of = Some(linked_item_ids_any_of.to_owned());
        self
    }
    pub fn linked_item_ids_all_of(mut self, linked_item_ids_all_of: &str) -> Self {
        self.linked_item_ids_all_of = Some(linked_item_ids_all_of.to_owned());
        self
    }
    pub fn linked_item_ids_none_of(mut self, linked_item_ids_none_of: &str) -> Self {
        self.linked_item_ids_none_of = Some(linked_item_ids_none_of.to_owned());
        self
    }
    pub fn schema_fields(mut self, schema_fields: SchemaFieldsQueryParam) -> Self {
        self.schema_fields = Some(schema_fields);
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
