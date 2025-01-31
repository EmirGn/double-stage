use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use uuid::Uuid;

pub mod models;
pub mod schema;

use self::models::{Chat, User, NewChat, NewUser};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connection to: {}", database_url))
}

pub fn create_user(conn: &mut PgConnection, username: &String) {
    use crate::database::schema::users;

    let id = Uuid::new_v4().to_string();
    let new_user = NewUser{ id: &id, username: &username.to_string() };
    
    let _ = diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn);
}

pub fn create_chat(conn: &mut PgConnection, id: &str, user_id: Option<&str>, title: &str, history: Option<&str>) {
    use crate::database::schema::chats;

    let uuid = Uuid::new_v4().to_string();

    let new_chat = NewChat{ id: &id, user_id: None, title, history };

    let _ = diesel::insert_into(chats::table)
        .values(new_chat)
        .execute(conn);
}