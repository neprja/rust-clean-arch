mod get_profile;

use crate::infrastructure::microservice_name::MicroserviceName;
use crate::infrastructure::repository::Repository;

#[derive(Clone)]
pub struct UseCase {
    repository: Repository,
    microservice_name: MicroserviceName,
}

impl UseCase {
    pub fn new(repository: Repository, microservice_name: MicroserviceName) -> Self {
        UseCase {
            repository,
            microservice_name,
        }
    }
}
