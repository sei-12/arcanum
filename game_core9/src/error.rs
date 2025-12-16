use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum Error {
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("Already game ended")]
    AlreadyGameEnded,

    #[error("NotFoundSkillInActionPattern")]
    NotFoundSkillInActionPattern,
}
