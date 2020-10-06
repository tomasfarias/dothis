use std::error;
use std::fmt;

use dothis::api::client::TodoistApiError;

#[derive(Debug)]
pub enum DothisError {
    ApiError(TodoistApiError),
    EmptyResponseError,
    UnknownResource,
    MissingCommand,
    UnknownCommand,
}

impl error::Error for DothisError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            DothisError::ApiError(ref err) => Some(err),
            DothisError::EmptyResponseError => None,
            DothisError::UnknownResource => None,
            DothisError::MissingCommand => None,
            DothisError::UnknownCommand => None,
        }
    }
}

impl fmt::Display for DothisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            DothisError::ApiError(ref err) => err.fmt(f),
            DothisError::MissingCommand => write!(f, "missing command"),
            // These three should specify what is unknown or what
            // field was empty. Also, I am not too fond of "resource" as a
            // word for tasks, projects, notes, and other todoist entities.
            DothisError::UnknownCommand => write!(f, "unknown command"),
            DothisError::EmptyResponseError => write!(f, "no resources found"),
            DothisError::UnknownResource => write!(f, "unknown resource"),
        }
    }
}

impl From<TodoistApiError> for DothisError {
    fn from(err: TodoistApiError) -> DothisError {
        DothisError::ApiError(err)
    }
}

impl From<DothisError> for i32 {
    fn from(err: DothisError) -> Self {
        match err {
            DothisError::ApiError(ref err) => 69,
            DothisError::EmptyResponseError => 69,
            DothisError::UnknownResource => 64,
            DothisError::MissingCommand => 64,
            DothisError::UnknownCommand => 64,
        }
    }
}
