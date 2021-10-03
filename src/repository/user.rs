use diesel::result::Error;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use crate::models::error::MyError;
use crate::models::token_request::TokenRequest;
use crate::models::token_response::TokenResponse;
use crate::models::user::{NewUser, ReqNewUser, ReqUpdUser, UpdUser, UserRead};
use crate::password_hash::PasswordHashService;
use crate::token::TokenService;

pub fn find_all(connection: &PgConnection) -> Result<Vec<UserRead>, Error> {
    use crate::schema::usr;

    usr::table
        .select((usr::username, usr::email, usr::birthday, usr::active))
        .load::<UserRead>(connection)
}

pub fn authenticate(
    connection: &PgConnection,
    token_service: &TokenService,
    password_hash_service: &PasswordHashService,
    token_req: TokenRequest,
) -> Result<TokenResponse, MyError> {
    use crate::schema::usr::dsl::*;

    let result: (String, String) = match usr
        .filter(username.eq(token_req.username.clone()))
        .select((password_hash, password_salt))
        .first(connection)
    {
        Ok(res) => res,
        Err(e) => return Err(MyError::new(format!("{}", e).as_str())),
    };

    if password_hash_service.verify_password(
        token_req.password.as_str(),
        result.1.as_str(),
        result.0.as_str(),
    ) {
        Ok(TokenResponse {
            access_token: token_service.signing(token_req.username.as_str()),
        })
    } else {
        Err(MyError::new("password is not valid"))
    }
}

pub fn create(
    connection: &PgConnection,
    password_hash_service: &PasswordHashService,
    obj: ReqNewUser,
) -> Result<UserRead, Error> {
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
        activation_token: &activation_token,
    };

    diesel::insert_into(usr::table)
        .values(&user)
        .returning((usr::username, usr::email, usr::birthday, usr::active))
        .get_result::<UserRead>(connection)
}

pub fn update(
    connection: &PgConnection,
    password_hash_service: &PasswordHashService,
    user_id: i32,
    obj: ReqUpdUser,
) -> Result<UserRead, MyError> {
    use crate::schema::usr::dsl::*;

    let result: (String, String) = match usr
        .filter(id.eq(user_id))
        .select((password_hash, password_salt))
        .first(connection)
    {
        Ok(res) => res,
        Err(e) => return Err(MyError::new(&format!("{}", e))),
    };

    if password_hash_service.verify_password(obj.old_password, result.1.as_str(), result.0.as_str())
    {
        let new_salt = PasswordHashService::generate_salt();
        let new_hash = password_hash_service.hash_password(obj.new_password, new_salt.as_str());
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
            Err(e) => Err(MyError::new(&format!("{}", e))),
        }
    } else {
        Err(MyError::new("old password is invalid"))
    }
}
