use serde_derive::*;
use serde_json::Value;
use std::fmt::Display;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Twitter API error!\n{0}params:\n{1}")]
    TwitterAPIError(TwitterAPIErrorMessage, String),
    #[error("Invalid json format.\n{0}\n{1}")]
    JsonParsingError(anyhow::Error, Value),
    #[error(transparent)]
    HTTPRequestError(reqwest::Error),
    /// Represents all other cases of `std::io::Error`.
    #[error(transparent)]
    Error(#[from] anyhow::Error),
}

#[derive(Serialize, Deserialize, Debug, Clone, Error)]
pub struct TwitterAPIErrorMessage {
    pub errors: Vec<APIErrorObject>,
}

impl Display for TwitterAPIErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for error in self.errors.clone() {
            writeln!(f, "Error code {}: {}", error.code, error.message)?;
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct APIErrorObject {
    pub code: i64,
    pub message: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn construct_apierror_object() {
        let erro_object = APIErrorObject {
            code: 100,
            message: "test error message".to_string(),
        };
        assert_eq!(erro_object.code, 100);
    }
}
