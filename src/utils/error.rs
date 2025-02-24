// error.rs - Fehlerbehandlung
#[derive(Debug)]
pub enum AppError {
    InvalidInput(String),
    InternalError(String),
}

impl warp::reject::Reject for AppError {}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::InvalidInput(msg) => write!(f, "Invalid Input: {}", msg),
            AppError::InternalError(msg) => write!(f, "Internal Error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}
