use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SignInDTO {
    pub username: String,
    pub password: String,
    pub code: String,
}
