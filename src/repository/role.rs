use diesel::{BelongingToDsl, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};

use crate::models::role::Role;
use crate::models::user::User;
use crate::models::user_role::UserRole;
use crate::repository::common::{PgConn, PgPool};

#[derive(Clone)]
pub struct RoleRepository(pub PgPool);

impl RoleRepository {
    fn get_conn(&self) -> PgConn {
        self.0.get().unwrap()
    }
    pub fn get_roles_by_user_id(&self, _id: i32) -> Vec<Role> {
        use crate::schema::role;
        use crate::schema::user_role::dsl::*;
        use crate::schema::usr;
        use diesel::expression::dsl::any;
        let conn = self.get_conn();

        let user = usr::table
            .filter(usr::id.eq(_id))
            .first::<User>(&conn)
            .unwrap();
        let user_role_ids = UserRole::belonging_to(&user).select(role_id);
        role::table
            .filter(role::id.eq(any(user_role_ids)))
            .load::<Role>(&conn)
            .unwrap_or(Vec::new())
    }
}
