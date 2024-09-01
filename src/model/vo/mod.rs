use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub mod jwt;
pub mod user_info;

use crate::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RespVO<T> {
    pub code: Option<String>,
    pub msg: Option<String>,
    pub data: Option<T>,
}

impl<T> RespVO<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    pub fn from_result(result: Result<T, Error>) -> Self {
        match result {
            Ok(data) => Self {
                code: Some(String::from("200")),
                msg: None,
                data: Some(data),
            },

            Err(e) => Self::from_error(e.to_string()),
        }
    }

    pub fn from_error_str(error: &str) -> Self {
        let code = String::from("500");

        Self {
            code: Some(code),
            msg: Some(error.to_string()),
            data: None,
        }
    }

    pub fn from_error(error: String) -> Self {
        let code = String::from("500");

        Self {
            code: Some(code),
            msg: Some(error),
            data: None,
        }
    }

    pub fn json(self) -> axum::Json<RespVO<T>> {
        axum::Json(self)
    }
}

impl<T> ToString for RespVO<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
