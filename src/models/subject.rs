use crate::schema::subject;

#[derive(Queryable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[table_name = "subject"]
pub struct Subject {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(AsChangeset, Debug, Deserialize)]
#[table_name = "subject"]
pub struct UpdSubject {
    pub name: Option<String>,
    pub description: Option<String>,
}
