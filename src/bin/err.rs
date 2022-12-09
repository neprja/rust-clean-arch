use anyhow::{bail, Context, Result};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("NotFound: {0}")]
    NotFound(String),
}

#[derive(Error, Debug)]
pub enum UsecaseError {
    #[error("RepositoryError: {0}")]
    RepositoryError(#[from] RepositoryError),
}

fn main() {
    match usecase_get_id().context("usecase_get_id") {
        Ok(_) => (),
        Err(err) => println!("{:#}", err),
    }
}

fn usecase_get_id() -> Result<String> {
    repository_get_id("0").context("repository_get_id")?;

    Ok("Ok!".to_string())
}

fn repository_get_id(str: &str) -> Result<String> {
    if str == "1" {
        return Ok("ok!!".to_string());
    } else {
        bail!(RepositoryError::NotFound("file.txt".to_string()))
    }
}
