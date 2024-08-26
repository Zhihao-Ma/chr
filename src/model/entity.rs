use rbatis::crud;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct SysUser {
    pub id: String,
    pub username: String,
    pub password: String,
}

crud!(SysUser {});
