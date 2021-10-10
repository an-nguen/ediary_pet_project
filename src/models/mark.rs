use chrono::NaiveDate;

use crate::schema::mark;

#[derive(Debug, Queryable, AsChangeset, Serialize)]
#[table_name = "mark"]
pub struct Mark {
    pub id: i32,
    pub student_id: i32,
    pub subject_id: i32,
    pub title: Option<String>,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
    pub _mark: String,
    pub author: Option<String>,
}

#[derive(Debug, Insertable, Deserialize)]
#[table_name = "mark"]
pub struct NewMark {
    pub student_id: i32,
    pub subject_id: i32,
    pub title: Option<String>,
    pub _mark: String,
}
