use diesel::{Queryable, Insertable};
use crate::schema::*;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub bio: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub bio: String,
}
