use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use uuid::Uuid;

pub mod models;
pub mod schema;

use self::models::{NewChat, NewUser, Chat};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connection to: {}", database_url))
}

#[allow(dead_code)]
pub fn create_new_user_handler_database(conn: &mut PgConnection, username: &String) {
    use crate::database::schema::users;

    let id = Uuid::new_v4().to_string();
    let new_user = NewUser{ id: &id, username: &username.to_string() };
    
    let _ = diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn);
}

pub fn create_new_chat_handler_database(conn: &mut PgConnection, id: &str, _user_id: Option<&str>, title: &str, history: Option<&str>) -> Result<usize, diesel::result::Error> {
    use crate::database::schema::chats;

    let new_chat = NewChat{ id: &id, user_id: None, title, history };

    let result = diesel::insert_into(chats::table)
        .values(new_chat)
        .execute(conn)?;

    Ok(result)
}

pub fn get_all_chat_history_handler_database(conn: &mut PgConnection, _user_id: Option<&str>) -> Result<Vec<Vec<String>>, diesel::result::Error> {
    use self::schema::chats::dsl::*;
    
    let titles = chats.select(title).load::<String>(conn)?;
    let ids = chats.select(id).load::<String>(conn)?;
    
    let title_id_vector = vec![titles, ids];
    Ok(title_id_vector)     
}

pub fn get_unit_chat_history_handler_database(conn: &mut PgConnection, chat_id: &str) -> Result<Chat, diesel::result::Error> {
    use crate::database::schema::chats::dsl::*;
    chats.find(chat_id).first(conn)
}

pub fn update_unit_chat_history_handler_database(conn: &mut PgConnection, chat_id: &str, new_history: &str) -> Result<usize, diesel::result::Error> {
    use crate::database::schema::chats::dsl::*;
    diesel::update(chats.filter(id.eq(chat_id)))
        .set(history.eq(new_history))
        .execute(conn)
}

pub fn delete_chat_handler_database(conn: &mut PgConnection, chat_id: &str) -> Result<usize, diesel::result::Error> {
    use crate::database::schema::chats::dsl::*;
    diesel::delete(chats.filter(id.eq(chat_id)))
        .execute(conn)
}