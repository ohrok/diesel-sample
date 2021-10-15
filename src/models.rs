use uuid::Uuid;

#[derive(Queryable)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub published: bool,
}

use super::schema::posts;

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub id: Uuid,
    pub title: &'a str,
    pub body: &'a str,
}
