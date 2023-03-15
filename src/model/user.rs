use diesel::{Queryable, Insertable, Identifiable, AsChangeset};
use crate::schema::*;

#[derive(Debug, Queryable, Identifiable, Clone, PartialEq, Eq)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub discord_userid: String,  // <-- as i64 (not u64) is the largest supported type in postgres
    pub bio: Option<String>,
}

#[derive(Debug, Insertable, AsChangeset, Clone, PartialEq, Eq)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub discord_userid: String,
    pub bio: Option<String>,
}
