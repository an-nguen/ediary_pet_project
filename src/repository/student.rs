use crate::models::error::ApiError;
use crate::models::student::{NewStudent, Student};
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};

pub fn find_all(connection: &PgConnection) -> Result<Vec<Student>, ApiError> {
    use crate::schema::student;

    match student::table.load::<Student>(connection) {
        Ok(res) => Ok(res),
        Err(err) => Err(ApiError::internal_server_error(Option::from(
            err.to_string(),
        ))),
    }
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

    match create!(student, connection, obj) {
        Ok(res) => Ok(res),
        Err(e) => Err(ApiError::internal_server_error(Option::from(format!(
            "{}",
            e
        )))),
    }
}

pub fn update(connection: &PgConnection, obj: Student) -> Result<Student, ApiError> {
    use crate::schema::student::dsl::*;

    if obj.first_name.is_empty() || obj.last_name.is_empty() {
        return Err(ApiError::bad_request(Option::from(
            "first name or last name cannot be empty".to_string(),
        )));
    }

    match diesel::update(student.filter(id.eq(obj.id)))
        .set(&obj)
        .get_result(connection)
    {
        Ok(res) => Ok(res),
        Err(err) => Err(ApiError::internal_server_error(Option::from(
            err.to_string(),
        ))),
    }
}
