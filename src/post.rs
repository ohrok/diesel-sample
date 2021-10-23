use super::schema::posts;
use uuid::Uuid;

#[derive(Queryable, Insertable)]
#[table_name = "posts"]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub user_id: Uuid,
}
