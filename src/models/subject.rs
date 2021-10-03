use crate::schema::subject;

#[derive(Queryable, Insertable, AsChangeset, Debug, Serialize, Deserialize)]
#[table_name = "subject"]
pub struct Subject {
    pub name: String,
    pub description: Option<String>
}