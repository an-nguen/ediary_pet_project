table! {
    mark (id) {
        id -> Int4,
        student_id -> Int4,
        subject_id -> Int4,
        title -> Nullable<Varchar>,
        created_at -> Date,
        updated_at -> Date,
        _mark -> Nullable<Varchar>,
        author -> Varchar,
    }
}

table! {
    role (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
    }
}

table! {
    stage (id) {
        id -> Int4,
        start_date -> Date,
        end_date -> Date,
        student_id -> Nullable<Int4>,
        _stage -> Varchar,
    }
}

table! {
    student (id) {
        id -> Int4,
        first_name -> Varchar,
        last_name -> Varchar,
        birth_date -> Nullable<Date>,
    }
}

table! {
    subject (id) {
        id -> Int4,
        name -> Varchar,
        description -> Nullable<Varchar>,
    }
}

table! {
    user_role (id) {
        id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
    }
}

table! {
    usr (id) {
        id -> Int4,
        username -> Varchar,
        password_hash -> Varchar,
        password_salt -> Varchar,
        email -> Varchar,
        birthday -> Date,
        active -> Bool,
        activation_token -> Nullable<Varchar>,
    }
}

joinable!(stage -> student (student_id));
joinable!(user_role -> role (role_id));
joinable!(user_role -> usr (user_id));

allow_tables_to_appear_in_same_query!(
    mark,
    role,
    stage,
    student,
    subject,
    user_role,
    usr,
);
