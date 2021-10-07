use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};

use crate::models::errors::ApiError;
use crate::models::subject::{Subject, UpdSubject};
use crate::models::DeletedCount;

pub fn find_all(connection: &PgConnection) -> Result<Vec<Subject>, ApiError> {
    use crate::schema::subject;

    find_all!(subject::table, Subject, connection)
}

pub fn create(connection: &PgConnection, obj: Subject) -> Result<Subject, ApiError> {
    use crate::schema::subject;

    create!(subject, connection, obj)
}

pub fn update(connection: &PgConnection, _id: i32, obj: UpdSubject) -> Result<Subject, ApiError> {
    use crate::schema::subject::dsl::*;

    match diesel::update(subject.filter(id.eq(_id)))
        .set(&obj)
        .get_result(connection)
    {
        Ok(res) => Ok(res),
        Err(e) => Err(ApiError::internal_server_error(Some(e.to_string()))),
    }
}

pub fn delete(conn: &PgConnection, _id: i32) -> Result<DeletedCount, ApiError> {
    use crate::schema::subject::dsl::*;

    match diesel::delete(subject.filter(id.eq(_id))).execute(conn) {
        Ok(count) => Ok(DeletedCount { count }),
        Err(e) => Err(ApiError::internal_server_error(Some(e.to_string()))),
    }
}
