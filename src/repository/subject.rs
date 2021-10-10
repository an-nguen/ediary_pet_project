use crate::errors::api_error::ApiError;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::models::subject::{Subject, UpdSubject};
use crate::models::DeletedCount;
use crate::repository::common::{PgPool, Repository, RepositoryResult};
use ediary_proc_macros::gen_repo_impl;

#[derive(Clone)]
#[gen_repo_impl]
pub struct SubjectRepository(pub PgPool);

impl Repository<i32, Subject, Subject, UpdSubject> for SubjectRepository {
    fn find_all(&self) -> RepositoryResult<Vec<Subject>> {
        use crate::schema::subject;
        let conn = self.get_conn();

        find_all!(subject::table, Subject, &conn)
    }

    fn get_one(&self, _id: i32) -> RepositoryResult<Subject> {
        use crate::schema::subject::dsl::*;
        let conn = self.get_conn();

        get_one!(Subject, subject, id.eq(_id), &conn)
    }

    fn create(&self, obj: Subject) -> RepositoryResult<Subject> {
        use crate::schema::subject;
        let conn = self.get_conn();

        create!(subject, &conn, obj)
    }

    fn update(&self, _id: i32, obj: UpdSubject) -> RepositoryResult<Subject> {
        use crate::schema::subject::dsl::*;
        let conn = self.get_conn();

        update!(subject.filter(id.eq(_id)), &conn, obj)
    }

    fn delete(&self, _id: i32) -> RepositoryResult<DeletedCount> {
        use crate::schema::subject::dsl::*;
        let conn = self.get_conn();

        delete!(subject.filter(id.eq(_id)), &conn)
    }
}
