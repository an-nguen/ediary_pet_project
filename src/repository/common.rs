#[macro_export]
macro_rules! find_all {
    ($table: expr, $result_type: ty, $conn: expr) => {
        match $table.load::<$result_type>($conn) {
            Ok(res) => Ok(res),
            Err(err) => Err(ApiError::internal_server_error(Some(err.to_string()))),
        }
    };
}

#[macro_export]
macro_rules! create {
    ($table_name: ident, $conn: expr, $obj: expr) => {
        match diesel::insert_into($table_name::table)
            .values(&$obj)
            .get_result($conn)
        {
            Ok(res) => Ok(res),
            Err(err) => Err(ApiError::internal_server_error(Some(err.to_string()))),
        }
    };
}

#[macro_export]
macro_rules! string_null_check {
    ($option: expr) => {
        if $option.is_none() {
            return Err(ApiError::bad_request(Some(format!(
                "{:?} cannot be empty!",
                $option
            ))));
        }

        if $option.clone().unwrap().is_empty() {
            return Err(ApiError::bad_request(Some(format!(
                "{:?} cannot be empty!",
                $option
            ))));
        }
    };
}

#[macro_export]
macro_rules! generate_token {
    ($token_service: expr, $username: expr, $exp: expr) => {
        match $token_service.signing($username, $exp) {
            Ok(t) => t,
            Err(e) => return Err(AuthError::Other(String::from(e.to_string()))),
        }
    };
}
