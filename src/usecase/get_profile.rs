use crate::usecase::UseCase;
use anyhow::Result;
use tracing_attributes::instrument;

impl UseCase {
    #[instrument(skip(self))]
    pub async fn get_profile(&self, id: String) -> Result<String> {
        self.microservice_name.get_profile(id).await
    }
}
