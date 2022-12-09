mod get_profile;

#[derive(Clone)]
pub struct Repository {
    state: String,
}

impl Repository {
    pub fn new() -> Self {
        Repository {
            state: "repository!".into(),
        }
    }
}
