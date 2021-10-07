use crate::models::errors::ApiError;
use crate::models::student::{NewStudent, Student, UpdStudent};
use crate::models::DeletedCount;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};

pub fn find_all(connection: &PgConnection) -> Result<Vec<Student>, ApiError> {
    use crate::schema::student;

    find_all!(student::table, Student, connection)
}

pub fn create(connection: &PgConnection, obj: NewStudent) -> Result<Student, ApiError> {
    use crate::schema::student;

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

    create!(student, connection, obj)
}

pub fn update(connection: &PgConnection, _id: i32, obj: UpdStudent) -> Result<Student, ApiError> {
    use crate::schema::student::dsl::*;

    string_null_check!(obj.first_name);
    string_null_check!(obj.last_name);

    match diesel::update(student.filter(id.eq(_id)))
        .set(&obj)
        .get_result(connection)
    {
        Ok(res) => Ok(res),
        Err(err) => Err(ApiError::internal_server_error(Option::from(
            err.to_string(),
        ))),
    }
}

pub fn delete(connection: &PgConnection, _id: i32) -> Result<DeletedCount, ApiError> {
    use crate::schema::student::dsl::*;

    match diesel::delete(student.filter(id.eq(_id))).execute(connection) {
        Ok(res) => Ok(DeletedCount { count: res }),
        Err(err) => Err(ApiError::internal_server_error(Some(err.to_string()))),
    }
}
