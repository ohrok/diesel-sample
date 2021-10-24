use diesel::prelude::*;
use diesel_sample::*;
use std::io::stdin;

fn main() {
    use diesel_sample::schema::posts::dsl::*;

    println!("Please enter the title to be deleted.");
    let target = {
        let mut s = String::new();
        stdin().read_line(&mut s).unwrap();
        s.trim().to_owned()
    };
    let pattern = format!("%{}%", target);

    let connection = establish_connection();
    let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
        .execute(&connection)
        .expect("Error deleting posts");

    println!("Deleted {} posts", num_deleted);
}
