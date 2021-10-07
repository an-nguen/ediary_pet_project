use crate::schema::role;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "role"]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}
