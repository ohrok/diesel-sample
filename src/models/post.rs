use super::super::schema::posts;
use super::user::User;
use uuid::Uuid;

#[derive(Queryable, Insertable, Identifiable, Associations)]
#[belongs_to(User)]
#[table_name = "posts"]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub user_id: Uuid,
}
