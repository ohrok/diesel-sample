use diesel::prelude::*;
use diesel_sample::*;
use models::post::Post;
use std::io::{stdin, Read};
use uuid::Uuid;

fn main() {
    use diesel_sample::schema::posts::dsl::{body, posts, title};

    println!("Please enter the id of the post you want to update.");
    let id = {
        let mut s = String::new();
        stdin().read_line(&mut s).unwrap();
        let s = s.trim();
        match Uuid::parse_str(s) {
            Ok(id) => id,
            Err(error) => {
                println!("Error parsing the entered post id to UUID: {}", error);
                return;
            }
        }
    };

    println!("\nPlease enter a new title.");
    let new_title = {
        let mut s = String::new();
        stdin().read_line(&mut s).unwrap();
        s.trim().to_owned()
    };

    println!(
        "\nOk! Let's write {} (Press {} when finished)\n",
        new_title, EOF
    );
    let new_body = {
        let mut s = String::new();
        stdin().read_to_string(&mut s).unwrap();
        s.trim().to_owned()
    };

    let connection = establish_connection();

    let post = diesel::update(posts.find(id))
        .set((title.eq(new_title), body.eq(new_body)))
        .get_result::<Post>(&connection)
        .expect(&format!("Unable to find post {}", id));
    println!("\nUpdated post {}", post.title);
}
