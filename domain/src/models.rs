use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use infrastructure::schema::users;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Ord, Eq, PartialEq, PartialOrd)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub created_at: NaiveDateTime,
    pub modified_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
}
