use chrono::{Duration, Utc};
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use crate::models::errors::{ApiError, AuthError};
use crate::models::token_request::TokenRequest;
use crate::models::token_response::TokenResponse;
use crate::models::user::{NewUser, ReqNewUser, ReqUpdUser, UpdUser, UserRead};
use crate::models::DeletedCount;
use crate::password_hash::PasswordHashService;
use crate::token::TokenService;

pub fn find_all(connection: &PgConnection) -> Result<Vec<UserRead>, ApiError> {
    use crate::schema::usr;

    match usr::table
        .select((usr::username, usr::email, usr::birthday, usr::active))
        .load::<UserRead>(connection)
    {
        Ok(res) => Ok(res),
        Err(e) => Err(ApiError::internal_server_error(Some(format!("{}", e)))),
    }
}

pub fn authenticate(
    connection: &PgConnection,
    token_service: &TokenService,
    password_hash_service: &PasswordHashService,
    token_req: TokenRequest,
) -> Result<TokenResponse, AuthError> {
    use crate::schema::usr::dsl::*;

    if let Some(admin) = token_service.config.admin.clone() {
        if let Some(password) = token_service.config.password.clone() {
            if token_req.username == admin && token_req.password == password {
                let access_token = generate_token!(
                    token_service,
                    token_req.username.as_str(),
                    (Utc::now() + Duration::hours(1)).timestamp() as u64
                );
                let refresh_token = generate_token!(
                    token_service,
                    token_req.username.as_str(),
                    (Utc::now() + Duration::days(1)).timestamp() as u64
                );
                return Ok(TokenResponse {
                    access_token,
                    refresh_token,
                });
            }
        }
    }

    let result: (String, bool) = match usr
        .filter(username.eq(token_req.username.clone()))
        .select((password_hash, active))
        .first(connection)
    {
        Ok(res) => res,
        Err(_) => return Err(AuthError::InvalidUsername),
    };

    if !result.1 {
        return Err(AuthError::NotActive);
    }

    if password_hash_service.verify_password(token_req.password.as_str(), result.0.as_str()) {
        let access_token = match token_service.signing(
            token_req.username.as_str(),
            (Utc::now() + Duration::hours(1)).timestamp() as u64,
        ) {
            Ok(t) => t,
            Err(e) => return Err(AuthError::Other(String::from(e.to_string()))),
        };
        let refresh_token = match token_service.signing(
            token_req.username.as_str(),
            (Utc::now() + Duration::days(1)).timestamp() as u64,
        ) {
            Ok(t) => t,
            Err(e) => return Err(AuthError::Other(String::from(e.to_string()))),
        };
        Ok(TokenResponse {
            access_token,
            refresh_token,
        })
    } else {
        Err(AuthError::InvalidPassword)
    }
}

pub fn create(
    connection: &PgConnection,
    password_hash_service: &PasswordHashService,
    obj: ReqNewUser,
    active: bool,
) -> Result<UserRead, ApiError> {
    use crate::schema::usr;

    let salt = PasswordHashService::generate_salt();
    let activation_token: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect();

    let hash = password_hash_service.hash_password(&obj.password, &salt);
    let user = NewUser {
        username: &obj.username,
        password_hash: &hash,
        password_salt: &salt,
        email: &obj.email,
        birthday: obj.birthday,
        active,
        activation_token: if active { &activation_token } else { "" },
    };

    match diesel::insert_into(usr::table)
        .values(&user)
        .returning((usr::username, usr::email, usr::birthday, usr::active))
        .get_result::<UserRead>(connection)
    {
        Ok(res) => Ok(res),
        Err(err) => Err(ApiError::internal_server_error(Some(err.to_string()))),
    }
}

pub fn update(
    connection: &PgConnection,
    password_hash_service: &PasswordHashService,
    user_id: i32,
    obj: ReqUpdUser,
) -> Result<UserRead, ApiError> {
    use crate::schema::usr::dsl::*;

    let result: String = match usr
        .filter(id.eq(user_id))
        .select(password_hash)
        .first(connection)
    {
        Ok(res) => res,
        Err(e) => return Err(ApiError::internal_server_error(Some(format!("{}", e)))),
    };

    if password_hash_service.verify_password(obj.old_password.as_str(), &result) {
        let new_salt = PasswordHashService::generate_salt();
        let new_hash =
            password_hash_service.hash_password(obj.new_password.as_str(), new_salt.as_str());
        let user = UpdUser {
            password_hash: new_hash.as_str(),
            password_salt: new_salt.as_str(),
            email: obj.email,
            birthday: obj.birthday,
        };
        match diesel::update(usr)
            .set(&user)
            .returning((username, email, birthday, active))
            .get_result::<UserRead>(connection)
        {
            Ok(res) => Ok(res),
            Err(e) => Err(ApiError::internal_server_error(Some(e.to_string()))),
        }
    } else {
        Err(ApiError::bad_request(Some("bad password".to_string())))
    }
}

pub fn delete(conn: &PgConnection, _id: i32) -> Result<DeletedCount, ApiError> {
    use crate::schema::usr::dsl::*;

    match diesel::delete(usr.find(_id)).execute(conn) {
        Ok(count) => Ok(DeletedCount { count }),
        Err(e) => Err(ApiError::internal_server_error(Some(e.to_string()))),
    }
}
