extern crate diesel;
extern crate diesel_sample;

use self::diesel::prelude::*;
use self::diesel_sample::*;
use self::models::Post;
use std::env::args;
use uuid::Uuid;

fn main() {
    use diesel_sample::schema::posts::dsl::{posts, published};

    let id = args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<Uuid>()
        .expect("Invalid ID");
    let connection = establish_connection();

    let post = diesel::update(posts.find(id))
        .set(published.eq(true))
        .get_result::<Post>(&connection)
        .expect(&format!("Unable to find post {}", id));
    println!("Published post {}", post.title);
}
