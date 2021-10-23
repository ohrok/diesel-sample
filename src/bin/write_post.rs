use diesel::*;
use diesel_sample::*;
use models::post::Post;
use std::io::{stdin, Read};
use uuid::Uuid;

fn main() {
    use schema::posts;
    let connection = establish_connection();

    println!("What would you like your title to be?");
    let title = {
        let mut s = String::new();
        stdin().read_line(&mut s).unwrap();
        s.trim().to_owned()
    };

    println!(
        "\nOk! Let's write {} (Press {} when finished)\n",
        title, EOF
    );
    let body = {
        let mut s = String::new();
        stdin().read_to_string(&mut s).unwrap();
        s.trim().to_owned()
    };

    println!("\n\nInput your user id!");
    let user_id = {
        let mut s = String::new();
        stdin().read_line(&mut s).unwrap();
        let s = s.trim();
        match Uuid::parse_str(s) {
            Ok(user_id) => user_id,
            Err(error) => {
                println!("Error parsing the entered user id to UUID: {}", error);
                return;
            }
        }
    };

    let new_post: Post = Post {
        id: Uuid::new_v4(),
        title: title,
        body: body,
        user_id: user_id,
    };

    let post: Post = match diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(&connection)
    {
        Ok(post) => post,
        Err(error) => {
            println!("Error saving new post: {}", error);
            return;
        }
    };

    println!("\nSaved draft {} with id {}", post.title, post.id);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";
