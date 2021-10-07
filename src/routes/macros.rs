#[macro_export]
macro_rules! user_has_role {
    ($token: expr, $role_str: expr) => {
        if !$token.has_role($role_str) {
            return Err(ApiError::new(403, Some(String::from("access denied"))));
        }
    };
}
