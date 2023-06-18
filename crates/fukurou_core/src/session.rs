use crate::{app_state::AppState, credentials::Credentials};
use thiserror::Error;

///
#[derive(Debug, Error, Clone)]
pub enum SessionError {
    /// Session has been disconnected. Please call `Session::connect'.
    /// If you have previously called `Session::connect` it means the session has been disconnected.
    /// This could be due to a network error.
    #[error("disconnected")]
    Disconnected,
    ///
    #[error("authentication error: {0}")]
    AuthenticationError(String),
    ///
    #[error("invalid argument: {0}")]
    InvalidArgument(String),
    ///
    #[error("invalid session config: {0}")]
    InvalidSessionConfig(String),
    ///
    #[error("internal error: {0}")]
    InternalError(String),
}

pub trait Session {
    fn new(app_state: &AppState) -> Result<Self, SessionError>
    where
        Self: std::marker::Sized;

    ///
    fn connect(&mut self, credentials: &Credentials) -> Result<(), SessionError>;
}
