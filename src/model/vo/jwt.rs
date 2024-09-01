use crate::error::{Error, Result};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Claims {
    pub id: String,
    pub username: String,
    pub permissions: Vec<String>,
    pub roles: Vec<String>,
    pub exp: usize,
}

impl Claims {
    pub fn create_token(&self, secert: &str) -> Result<String> {
        return match encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(secert.as_ref()),
        ) {
            Ok(t) => Ok(t),
            Err(_) => Err(Error::from("jwt encode fial!")),
        };
    }
}
