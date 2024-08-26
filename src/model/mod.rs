pub mod dto;
pub mod entity;
pub mod vo;
use rbatis::crud;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Myorder {
    pub id: i32,
    pub user_id: Option<i32>,
    pub item_id: Option<i32>,
    pub price: Option<i32>,
    pub count: Option<i32>,
    pub current: Option<String>,
    pub sub_time: Option<i64>,
    pub pay_time: Option<i64>,
    pub inventory_state: i32,
    pub description: Option<String>,
}

crud!(Myorder {});
