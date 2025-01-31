use diesel::{prelude::*, AsChangeset};
use serde::{Deserialize, Serialize};

// Model for the 'chats' table
#[derive(Debug, Queryable, Selectable, Identifiable, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = crate::database::schema::chats)]
pub struct Chat {
    pub id: String,
    pub user_id: Option<String>, // nullable Text column
    pub title: String, // Varchar(50) column
    pub history: Option<String>, // nullable Text column
}

// Model for the 'users' table
#[derive(Debug, Queryable, Selectable, Identifiable, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = crate::database::schema::users)]
pub struct User {
    pub id: String, // Text column (not nullable)
    pub username: String, // Varchar(50) column
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::database::schema::users)] // Use schema module for table name
pub struct NewUser<'a> {
    pub id: &'a str,
    pub username: &'a str,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::database::schema::chats)] // Use schema module for table name
pub struct NewChat<'a> {
    pub id: &'a str,
    pub user_id: Option<&'a str>,
    pub title: &'a str,
    pub history: Option<&'a str>,
}