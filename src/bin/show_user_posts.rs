use diesel::prelude::*;
use diesel_sample::*;
use models::post::Post;
use models::user::User;
use std::io::stdin;
use uuid::Uuid;

fn main() {
    use diesel_sample::schema::users::dsl::users;
    let connection = establish_connection();

    println!("Please enter your user id.");
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

    let user = users
        .find(user_id)
        .get_result::<User>(&connection)
        .expect(&format!("Unable to find user {}", user_id));

    let results = Post::belonging_to(&user)
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("\nUser: {}", user.username);
    println!("Displaying {} posts", results.len());
    for post in results {
        println!("\n{}", post.title);
        println!("----------");
        println!("{}", post.body);
        println!("----------");
    }
}
