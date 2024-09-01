use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct UserInfoVO {
    pub id: String,
    pub username: String,
    pub permissions: Vec<String>,
    pub roles: Vec<String>,
}
