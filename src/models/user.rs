use super::super::schema::users;
use uuid::Uuid;

#[derive(Queryable, Insertable, Identifiable)]
#[table_name = "users"]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub username: String,
}
