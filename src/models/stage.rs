use chrono::NaiveDate;
use crate::schema::stage;

#[derive(Queryable, AsChangeset, Debug, Serialize)]
#[table_name = "stage"]
pub struct Stage {
    pub id: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub student_id: i32,
    pub _stage: String
}

#[derive(Debug, Insertable, Deserialize)]
#[table_name = "stage"]
pub struct NewStage {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub student_id: i32,
    pub _stage: String
}