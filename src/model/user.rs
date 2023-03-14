use diesel::{Queryable, Insertable};
use crate::schema::*;

#[derive(Debug, Queryable, Clone, PartialEq, Eq)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub bio: Option<String>,
}

#[derive(Debug, Insertable, Clone, PartialEq, Eq)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub bio: String,
}
