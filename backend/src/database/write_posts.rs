use crate::database::{establish_connection, create_post};
use std::io::{stdin, Read};

fn main() {
    let connection = &mut establish_connection();

    let mut title = String::new();
    let mut body = String::new();

    println!("What would you like your title to be?");
    stdin().read_line(&mut title).unwrap();
    let title = title.trim_end();

    println!("\nOk, let's write {title} (Press Ctrl+D when finished)\n");
    stdin().read_to_string(&mut body).unwrap();

    let post = create_post(connection, title, &body);
    println!("\nSaved draft {title} with id {}", post.id);
}

#[cfg(test)]
mod tests {
    use super::*;
    use diesel::prelude::*;
    use diesel::test_transaction;

    #[test]
    fn test_create_post() {
        let connection = &mut establish_connection();
        
        test_transaction(connection, |conn| {
            let test_title = "Test Post";
            let test_body = "Test Content";

            let post = create_post(conn, test_title, test_body);

            assert_eq!(post.title, test_title);
            assert_eq!(post.body, test_body);
            assert_eq!(post.published, false);

            Ok(())
        });
    }
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";