use crate::logic::{AuthError, ConfigError, GithubError};
use sea_orm::DbErr;
use thiserror::Error;

mod deployment;
mod handlers;
mod instance;

pub use deployment::Deployment;
pub use handlers::*;
pub use instance::{Instance, InstanceDeployment};

#[derive(Error, Debug)]
pub enum DeployError {
    #[error("auth error: {0}")]
    Auth(#[from] AuthError),
    #[error("config error: {0}")]
    Config(#[from] ConfigError),
    #[error("github error: {0}")]
    Github(#[from] GithubError),
    #[error("instance with name `{0}` already exists")]
    InstanceExists(String),
    #[error("instance with id `{0}` not found")]
    InstanceNotFound(String),
    #[error("deployment not found")]
    DeploymentNotFound,
    #[error("db error: {0}")]
    Db(#[from] DbErr),
    #[error("internal error: {0}")]
    Internal(#[from] anyhow::Error),
}
