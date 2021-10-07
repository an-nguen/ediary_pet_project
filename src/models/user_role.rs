use super::role::Role;
use super::user::User;
use crate::schema::user_role;

#[derive(Debug, Identifiable, Queryable, Associations, PartialEq)]
#[belongs_to(User)]
#[belongs_to(Role)]
#[table_name = "user_role"]
pub struct UserRole {
    pub id: i32,
    pub user_id: i32,
    pub role_id: i32,
}
