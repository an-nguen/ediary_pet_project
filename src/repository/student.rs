use crate::errors::api_error::ApiError;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::models::student::{NewStudent, Student, UpdStudent};
use crate::models::DeletedCount;
use crate::repository::common::{PgPool, Repository, RepositoryResult};
use crate::schema::student;

use ediary_proc_macros::gen_repo_impl;

#[derive(Clone)]
#[gen_repo_impl]
pub struct StudentRepository(pub PgPool);

impl Repository<i32, Student, NewStudent, UpdStudent> for StudentRepository {
    fn find_all(&self) -> RepositoryResult<Vec<Student>> {
        let conn = self.get_conn();

        find_all!(student::table, Student, &conn)
    }

    fn get_one(&self, _id: i32) -> RepositoryResult<Student> {
        let conn = self.get_conn();
        get_one!(Student, student::table, student::id.eq(_id), &conn)
    }

    fn create(&self, obj: NewStudent) -> RepositoryResult<Student> {
        let conn = self.get_conn();

        if obj.first_name.is_none() || obj.last_name.is_none() {
            return Err(ApiError::bad_request(Option::from(
                "first name or last name cannot be empty".to_string(),
            )));
        }

        if obj.first_name.clone().unwrap().is_empty() || obj.last_name.clone().unwrap().is_empty() {
            return Err(ApiError::bad_request(Option::from(
                "first name or last name cannot be empty".to_string(),
            )));
        }

        create!(student, &conn, obj)
    }

    fn update(&self, _id: i32, obj: UpdStudent) -> RepositoryResult<Student> {
        use crate::schema::student::dsl::*;
        let conn = self.get_conn();

        string_null_check!(obj.first_name);
        string_null_check!(obj.last_name);

        update!(student.filter(id.eq(_id)), &conn, obj)
    }

    fn delete(&self, _id: i32) -> RepositoryResult<DeletedCount> {
        use crate::schema::student::dsl::*;
        let conn = self.get_conn();

        delete!(student.filter(id.eq(_id)), &conn)
    }
}
