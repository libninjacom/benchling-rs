use serde_json::json;
use crate::model::*;
use crate::BenchlingClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
pub struct UpdateContainerRequest<'a> {
    pub(crate) client: &'a BenchlingClient,
    pub container_id: String,
    pub fields: Fields,
    pub name: String,
    pub parent_storage_id: String,
    pub project_id: Option<String>,
    pub quantity: ContainerQuantity,
    pub volume: DeprecatedContainerVolumeForInput,
}
impl<'a> UpdateContainerRequest<'a> {
    pub async fn send(self) -> anyhow::Result<Container> {
        let mut r = self
            .client
            .client
            .patch(
                &format!("/containers/{container_id}", container_id = self.container_id),
            );
        r = r.push_json(json!({ "fields" : self.fields }));
        r = r.push_json(json!({ "name" : self.name }));
        r = r.push_json(json!({ "parentStorageId" : self.parent_storage_id }));
        if let Some(ref unwrapped) = self.project_id {
            r = r.push_json(json!({ "projectId" : unwrapped }));
        }
        r = r.push_json(json!({ "quantity" : self.quantity }));
        r = r.push_json(json!({ "volume" : self.volume }));
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
    pub fn project_id(mut self, project_id: &str) -> Self {
        self.project_id = Some(project_id.to_owned());
        self
    }
}
pub struct UpdateContainerRequired<'a> {
    pub container_id: &'a str,
    pub fields: Fields,
    pub name: &'a str,
    pub parent_storage_id: &'a str,
    pub quantity: ContainerQuantity,
    pub volume: DeprecatedContainerVolumeForInput,
}
impl<'a> UpdateContainerRequired<'a> {}
