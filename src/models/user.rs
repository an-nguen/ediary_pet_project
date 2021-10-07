use crate::schema::usr;
use chrono::NaiveDate;

#[derive(Debug, Queryable, Insertable, Identifiable)]
#[table_name = "usr"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub password_salt: String,
    pub email: String,
    pub birthday: NaiveDate,
    pub active: bool,
    pub activation_token: Option<String>,
}

#[derive(Debug, Queryable, Serialize)]
pub struct UserRead {
    pub username: String,
    pub email: String,
    pub birthday: NaiveDate,
    pub active: bool,
}

#[derive(Debug, Deserialize)]
pub struct ReqNewUser {
    pub username: String,
    pub password: String,
    pub email: String,
    pub birthday: NaiveDate,
}

#[derive(Debug, Insertable)]
#[table_name = "usr"]
pub struct NewUser<'r> {
    pub username: &'r str,
    pub password_hash: &'r str,
    pub password_salt: &'r str,
    pub email: &'r str,
    pub birthday: NaiveDate,
    pub active: bool,
    pub activation_token: &'r str,
}

#[derive(Debug, Deserialize)]
pub struct ReqUpdUser {
    pub old_password: String,
    pub new_password: String,
    pub email: Option<String>,
    pub birthday: Option<NaiveDate>,
}

#[derive(Debug, AsChangeset)]
#[table_name = "usr"]
pub struct UpdUser<'r> {
    pub password_hash: &'r str,
    pub password_salt: &'r str,
    pub email: Option<String>,
    pub birthday: Option<NaiveDate>,
}
