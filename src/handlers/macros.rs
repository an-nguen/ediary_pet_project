#[macro_export]
macro_rules! user_has_role {
    ($token_auth: expr, $role_str: expr) => {
        if !$token_auth.has_role($role_str) {
            return AuthError::AccessDenied.error_response();
        }
    };
}

#[macro_export]
macro_rules! user_has_roles {
    ($token: expr, $role_vec: expr) => {
        let mut has_role = false;
        for r in $role_vec.iter() {
            if $token.has_role(r) {
                has_role = true;
                break;
            }
        }
        if !has_role {
            return AuthError::AccessDenied.error_response();
        }
    };
}

macro_rules! find_all {
    ($repo: ident) => {
        match $repo.find_all() {
            Ok(res) => HttpResponse::Ok().json(res),
            Err(e) => e.error_response(),
        }
    };
}

macro_rules! create {
    ($repo: ident, $obj: expr) => {
        match $repo.create($obj) {
            Ok(res) => HttpResponse::Ok().json(res),
            Err(e) => e.error_response(),
        }
    };
}

macro_rules! update {
    ($repo: ident, $id: expr, $obj: expr) => {
        match $repo.update($id, $obj) {
            Ok(res) => HttpResponse::Ok().json(res),
            Err(e) => e.error_response(),
        }
    };
}

macro_rules! delete {
    ($repo: ident, $id: expr) => {
        match $repo.delete($id) {
            Ok(res) => HttpResponse::Ok().json(res),
            Err(e) => e.error_response(),
        }
    };
}
