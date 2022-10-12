use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct CreateFolderRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub name: String,
    pub parent_folder_id: String,
}
impl<'a> CreateFolderRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Folder> {
        let mut r = self.client.client.post("/folders");
        r = r.push_json(json!({ "name" : self.name }));
        r = r.push_json(json!({ "parentFolderId" : self.parent_folder_id }));
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
