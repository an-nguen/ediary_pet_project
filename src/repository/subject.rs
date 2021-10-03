use crate::models::subject::Subject;
use diesel::{PgConnection, RunQueryDsl, QueryDsl, ExpressionMethods};
use diesel::result::Error;

pub fn find_all(connection: &PgConnection) -> Result<Vec<Subject>, Error> {
    use crate::schema::subject;

    subject::table.load::<Subject>(connection)
}

pub fn create(connection: &PgConnection, obj: Subject) -> Result<Subject, Error> {
    use crate::schema::subject;

    create!(subject, connection, obj)
}

pub fn update(connection: &PgConnection, obj: Subject) -> Result<Subject, Error> {
    use crate::schema::subject::dsl::*;

    diesel::update(subject.filter(name.eq(obj.name.as_str())))
        .set(&obj)
        .get_result(connection)
}