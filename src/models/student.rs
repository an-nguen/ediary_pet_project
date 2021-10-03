use chrono::NaiveDate;
use crate::schema::student;

#[derive(Queryable, AsChangeset, Debug, Serialize)]
#[table_name = "student"]
pub struct Student {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub birth_date: Option<NaiveDate>,
}

#[derive(Debug, Insertable, Deserialize)]
#[table_name = "student"]
pub struct NewStudent {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub birth_date: Option<NaiveDate>,
}