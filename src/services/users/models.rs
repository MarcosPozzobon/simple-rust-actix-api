use serde::{Deserialize, Serialize};
use sqlx::FromRow;


#[derive(Serialize, Deserialize)]
pub struct CreatedUser {
    pub name : String,
    pub email : String
}

#[derive(Serialize, Deserialize)]
pub struct AllUsers {
    pub id : i32,
    pub name : String,
    pub email : String,
    pub password : String
}

#[derive(Serialize, Deserialize)]
pub struct RegisterUser{
    pub name : String,
    pub email : String,
    pub password : String
}

#[derive(Serialize, Deserialize)]
pub struct UpdateUser{
    pub name : String,
    pub email : String,
    pub password : String
}





