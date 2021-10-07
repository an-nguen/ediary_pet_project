use diesel::{BelongingToDsl, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};

use crate::models::role::Role;
use crate::models::user::User;
use crate::models::user_role::UserRole;

pub fn get_roles_by_username(connection: &PgConnection, _username: &str) -> Vec<Role> {
    use crate::schema::role;
    use crate::schema::user_role::dsl::*;
    use crate::schema::usr;
    use crate::schema::usr::dsl::*;
    use diesel::expression::dsl::any;

    let user = usr::table
        .filter(username.eq(_username))
        .first::<User>(connection)
        .unwrap();
    let user_role_ids = UserRole::belonging_to(&user).select(role_id);
    role::table
        .filter(role::id.eq(any(user_role_ids)))
        .load::<Role>(connection)
        .unwrap()
}
