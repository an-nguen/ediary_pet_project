use diesel::r2d2::ConnectionManager;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use r2d2::{Pool, PooledConnection};

use crate::errors::api_error::ApiError;
use crate::hash_helpers::{generate_random_string, hash_password, verify_password};
use crate::models::user::{NewUser, ReqNewUser, ReqUpdUser, UpdUser, UserRead};
use crate::models::DeletedCount;
use crate::repository::common::Repository;
use crate::repository::common::RepositoryResult;

#[derive(Clone)]
pub struct UserRepository {
    pub pg_pool: Pool<ConnectionManager<PgConnection>>,
}

fn create(connection: &PgConnection, obj: ReqNewUser, active: bool) -> Result<UserRead, ApiError> {
    use crate::schema::usr;

    let activation_token = generate_random_string(32);

    let hash = hash_password(&obj.password);
    let user = NewUser {
        username: &obj.username,
        password_hash: &hash,
        email: &obj.email,
        birthday: obj.birthday,
        active,
        activation_token: if active { &activation_token } else { "" },
    };

    match diesel::insert_into(usr::table)
        .values(&user)
        .returning((
            usr::id,
            usr::username,
            usr::email,
            usr::birthday,
            usr::active,
        ))
        .get_result::<UserRead>(connection)
    {
        Ok(res) => Ok(res),
        Err(err) => Err(ApiError::internal_server_error(Some(err.to_string()))),
    }
}

impl UserRepository {
    pub fn get_conn(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.pg_pool.get().unwrap()
    }
    pub fn create_active(&self, obj: ReqNewUser) -> RepositoryResult<UserRead> {
        let conn = self.get_conn();
        create(&conn, obj, true)
    }
    pub fn get_by_username(&self, _username: &str) -> RepositoryResult<(i32, String, bool)> {
        let conn = self.get_conn();
        use crate::schema::usr::dsl::*;

        let result: (i32, String, bool) = match usr
            .filter(username.eq(_username))
            .select((id, password_hash, active))
            .first(&conn)
        {
            Ok(res) => res,
            Err(_) => return Err(ApiError::internal_server_error(Some("".to_string()))),
        };
        return Ok(result);
    }
}

impl Repository<i32, UserRead, ReqNewUser, ReqUpdUser> for UserRepository {
    fn find_all(&self) -> RepositoryResult<Vec<UserRead>> {
        use crate::schema::usr;
        let conn = self.get_conn();

        match usr::table
            .select((
                usr::id,
                usr::username,
                usr::email,
                usr::birthday,
                usr::active,
            ))
            .load::<UserRead>(&conn)
        {
            Ok(res) => Ok(res),
            Err(e) => Err(ApiError::internal_server_error(Some(format!("{}", e)))),
        }
    }

    fn get_one(&self, _id: i32) -> RepositoryResult<UserRead> {
        use crate::schema::usr::dsl::*;
        let conn = self.get_conn();

        match usr
            .filter(id.eq(_id))
            .select((id, username, email, birthday, active))
            .first::<UserRead>(&conn)
        {
            Ok(user) => Ok(user),
            Err(e) => Err(ApiError::internal_server_error(Some(format!("{}", e)))),
        }
    }

    fn create(&self, obj: ReqNewUser) -> RepositoryResult<UserRead> {
        let conn = self.get_conn();
        create(&conn, obj, false)
    }

    fn update(&self, _id: i32, obj: ReqUpdUser) -> RepositoryResult<UserRead> {
        use crate::schema::usr::dsl::*;
        let conn = self.get_conn();

        let result: String = match usr.filter(id.eq(_id)).select(password_hash).first(&conn) {
            Ok(res) => res,
            Err(e) => return Err(ApiError::internal_server_error(Some(format!("{}", e)))),
        };

        if verify_password(obj.old_password.as_str(), &result) {
            let new_hash = hash_password(obj.new_password.as_str());
            let user = UpdUser {
                password_hash: new_hash.as_str(),
                email: obj.email,
                birthday: obj.birthday,
            };
            match diesel::update(usr)
                .set(&user)
                .returning((id, username, email, birthday, active))
                .get_result::<UserRead>(&conn)
            {
                Ok(res) => Ok(res),
                Err(e) => Err(ApiError::internal_server_error(Some(e.to_string()))),
            }
        } else {
            Err(ApiError::bad_request(Some("bad password".to_string())))
        }
    }

    fn delete(&self, _id: i32) -> RepositoryResult<DeletedCount> {
        use crate::schema::usr::dsl::*;
        let conn = self.get_conn();

        match diesel::delete(usr.find(_id)).execute(&conn) {
            Ok(count) => Ok(DeletedCount { count }),
            Err(e) => Err(ApiError::internal_server_error(Some(e.to_string()))),
        }
    }
}
