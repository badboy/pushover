use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("JSON: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Io: {0}")]
    Io(#[from] std::io::Error),

    #[error("Reqwest: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Pushover error. status={status}")]
    PushoverError {
        status: i32,
        errors: Vec<String>,
        request: String,
    },
}
