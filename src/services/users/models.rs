use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct AllUsers{
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String
}

#[derive(Serialize, Deserialize)]
pub struct RegisterUser{
    pub name: String,
    pub email: String,
    pub password: String
}

#[derive(Serialize, Deserialize)]
pub struct UpdateUser{
    pub name: String,
    pub email: String,
    pub password: String,
}

