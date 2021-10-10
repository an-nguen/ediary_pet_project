use crate::errors::api_error::ApiError;
use crate::models::stage::{NewStage, Stage, UpdStage};
use crate::models::DeletedCount;
use crate::schema::stage;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::repository::common::{PgPool, Repository, RepositoryResult};
use ediary_proc_macros::gen_repo_impl;

#[derive(Clone)]
#[gen_repo_impl]
pub struct StageRepository(pub PgPool);

impl Repository<i32, Stage, NewStage, UpdStage> for StageRepository {
    fn find_all(&self) -> RepositoryResult<Vec<Stage>> {
        let conn = self.get_conn();
        find_all!(stage::table, Stage, &conn)
    }

    fn get_one(&self, _id: i32) -> RepositoryResult<Stage> {
        let conn = self.get_conn();
        get_one!(Stage, stage::table, stage::id.eq(_id), &conn)
    }

    fn create(&self, obj: NewStage) -> RepositoryResult<Stage> {
        let conn = self.get_conn();
        create!(stage, &conn, obj)
    }

    fn update(&self, _id: i32, obj: UpdStage) -> RepositoryResult<Stage> {
        use crate::schema::stage::dsl::*;
        let conn = self.get_conn();

        update!(stage.filter(id.eq(_id)), &conn, obj)
    }

    fn delete(&self, _id: i32) -> RepositoryResult<DeletedCount> {
        use crate::schema::stage::dsl::*;
        let conn = self.get_conn();

        delete!(stage.filter(id.eq(_id)), &conn)
    }
}
