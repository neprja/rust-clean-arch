use crate::infrastructure::repository::Repository;
use anyhow::Result;
use tracing_attributes::instrument;

impl Repository {
    #[instrument(skip(self))]
    pub async fn get_profile(&self, id: String) -> Result<String> {
        Ok("ok!".to_string())
    }
}
