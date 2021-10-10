use crate::errors::api_error::ApiError;
use crate::models::DeletedCount;

macro_rules! find_all {
    ($table: expr, $result_type: ty, $conn: expr) => {
        match $table.load::<$result_type>($conn) {
            Ok(res) => Ok(res),
            Err(err) => Err(ApiError::internal_server_error(Some(err.to_string()))),
        }
    };
}

macro_rules! get_one {
    ($t: ty, $table: expr, $predicate: expr, $conn: expr) => {
        match $table.filter($predicate).first::<$t>($conn) {
            Ok(res) => Ok(res),
            Err(err) => Err(ApiError::internal_server_error(Some(err.to_string()))),
        }
    };
}

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

/// Update entity and returns `Result<<your_entity_type>, ApiError>`
///
/// # Parameters
///
/// 1) filter expression - Diesel's QueryDsl like <table>.filter(<table_column>.eq(<val>))
/// 2) diesel::PgConnection connection
/// 3) model instance
///
/// # Example
///
/// ```
/// update!(user.filter(id.eq(_id)), connection, user)
///
/// ```
macro_rules! update {
    ($filter: expr, $conn: expr, $obj: expr) => {
        match diesel::update($filter).set(&$obj).get_result($conn) {
            Ok(res) => Ok(res),
            Err(err) => Err(ApiError::internal_server_error(Option::from(
                err.to_string(),
            ))),
        }
    };
}

macro_rules! delete {
    ($filter: expr, $conn: expr) => {
        match diesel::delete($filter).execute($conn) {
            Ok(res) => Ok(DeletedCount { count: res }),
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

pub type RepositoryResult<T> = Result<T, ApiError>;
pub type PgPool = r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;
pub type PgConn = r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

pub trait Repository<PK, GetObj, NewObj, UpdObj> {
    fn find_all(&self) -> RepositoryResult<Vec<GetObj>>;
    fn get_one(&self, id: PK) -> RepositoryResult<GetObj>;
    fn create(&self, obj: NewObj) -> RepositoryResult<GetObj>;
    fn update(&self, id: PK, obj: UpdObj) -> RepositoryResult<GetObj>;
    fn delete(&self, id: PK) -> RepositoryResult<DeletedCount>;
}
