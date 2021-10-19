use diesel::*;
use diesel_sample::*;
use std::io::stdin;
use user::User;
use uuid::Uuid;

fn main() {
    use schema::users;
    let connection = establish_connection();

    // input name
    println!("Input your name!");
    let name = {
        let mut s = String::new();
        stdin().read_line(&mut s).unwrap();
        s.trim().to_owned()
    };

    //  input username
    println!("\nInput your userame!");
    let username = {
        let mut s = String::new();
        stdin().read_line(&mut s).unwrap();
        s.trim().to_owned()
    };

    let new_user = User {
        id: Uuid::new_v4(),
        name: name,
        username: username,
    };

    let user: User = match diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(&connection)
    {
        Ok(user) => user,
        Err(error) => {
            println!("Error saving new user: {}", error);
            return;
        }
    };

    println!("\nSaved user {} with id {}", user.name, user.id);
}
