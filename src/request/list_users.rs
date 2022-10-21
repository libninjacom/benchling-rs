use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct ListUsersRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub ids: Option<String>,
    pub name: Option<String>,
    pub name_includes: Option<String>,
    pub names_any_of: Option<String>,
    pub names_any_of_case_sensitive: Option<String>,
    pub modified_at: Option<String>,
    pub member_of: Option<String>,
    pub admin_of: Option<String>,
    pub handles: Option<String>,
    pub password_last_changed_at: Option<String>,
    pub page_size: Option<i64>,
    pub next_token: Option<String>,
    pub sort: Option<String>,
}
impl<'a> ListUsersRequest<'a> {
    pub async fn send(self) -> anyhow::Result<UsersPaginatedList> {
        let mut r = self.client.client.get("/users");
        if let Some(ref unwrapped) = self.ids {
            r = r.push_query("ids", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.name {
            r = r.push_query("name", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.name_includes {
            r = r.push_query("nameIncludes", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.names_any_of {
            r = r.push_query("names.anyOf", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.names_any_of_case_sensitive {
            r = r.push_query("names.anyOf.caseSensitive", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.modified_at {
            r = r.push_query("modifiedAt", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.member_of {
            r = r.push_query("memberOf", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.admin_of {
            r = r.push_query("adminOf", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.handles {
            r = r.push_query("handles", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.password_last_changed_at {
            r = r.push_query("passwordLastChangedAt", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.page_size {
            r = r.push_query("pageSize", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.next_token {
            r = r.push_query("nextToken", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.sort {
            r = r.push_query("sort", &unwrapped.to_string());
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
    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_owned());
        self
    }
    pub fn name_includes(mut self, name_includes: &str) -> Self {
        self.name_includes = Some(name_includes.to_owned());
        self
    }
    pub fn names_any_of(mut self, names_any_of: &str) -> Self {
        self.names_any_of = Some(names_any_of.to_owned());
        self
    }
    pub fn names_any_of_case_sensitive(
        mut self,
        names_any_of_case_sensitive: &str,
    ) -> Self {
        self.names_any_of_case_sensitive = Some(names_any_of_case_sensitive.to_owned());
        self
    }
    pub fn modified_at(mut self, modified_at: &str) -> Self {
        self.modified_at = Some(modified_at.to_owned());
        self
    }
    pub fn member_of(mut self, member_of: &str) -> Self {
        self.member_of = Some(member_of.to_owned());
        self
    }
    pub fn admin_of(mut self, admin_of: &str) -> Self {
        self.admin_of = Some(admin_of.to_owned());
        self
    }
    pub fn handles(mut self, handles: &str) -> Self {
        self.handles = Some(handles.to_owned());
        self
    }
    pub fn password_last_changed_at(mut self, password_last_changed_at: &str) -> Self {
        self.password_last_changed_at = Some(password_last_changed_at.to_owned());
        self
    }
    pub fn page_size(mut self, page_size: i64) -> Self {
        self.page_size = Some(page_size);
        self
    }
    pub fn next_token(mut self, next_token: &str) -> Self {
        self.next_token = Some(next_token.to_owned());
        self
    }
    pub fn sort(mut self, sort: &str) -> Self {
        self.sort = Some(sort.to_owned());
        self
    }
}
