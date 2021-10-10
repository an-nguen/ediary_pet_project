pub mod mark;
pub mod role;
pub mod stage;
pub mod student;
pub mod subject;
pub mod user;
pub mod user_role;

#[derive(Debug, Serialize)]
pub struct DeletedCount {
    pub count: usize,
}
