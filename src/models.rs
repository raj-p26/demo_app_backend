use sqlx::FromRow;
use serde::{ Serialize, Deserialize };

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RequestUser {
    pub name: String,
    pub age: i32
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ResponseUser {
    pub id: String,
    pub name: String,
    pub age: i32
}
