use diesel::prelude::*;
use diesel_sample::*;
use models::post::Post;
use models::user::User;

fn main() {
    use diesel_sample::schema::posts::dsl::*;
    use diesel_sample::schema::users::dsl::*;

    let connection = establish_connection();
    let results = posts
        .limit(5)
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("\n{}", post.title);
        println!("----------");
        println!("{}", post.body);
        println!("----------");

        let user: User = users.find(post.user_id).get_result(&connection).unwrap();
        println!("by {}", user.username);
    }
}
