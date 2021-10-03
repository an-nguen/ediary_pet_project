#[macro_export]
macro_rules! create {
    ($table_name: ident, $conn: expr, $obj: expr) => {
        diesel::insert_into($table_name::table)
        .values(&$obj)
        .get_result($conn)
    };
}
