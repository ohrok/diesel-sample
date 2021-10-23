use diesel::*;
use diesel_sample::*;
use models::user::User;

fn main() {
    use diesel_sample::schema::users::dsl::*;

    let connection = establish_connection();
    let results = users
        .limit(5)
        .load::<User>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{} (username: {})", user.name, user.username);
    }
}
