# school e-diary application (pet project)

**Backend** - REST API server written in Rust using **Actix-web** framework + **Diesel** ORM with Token-based authentication   
**Frontend** - ?

# Structure

````   
.
├── Cargo.toml
├── configs
│   ├── Default.toml
│   └── Production.toml
├── diesel.toml
├── README.md
└── src
    ├── config.rs
    ├── db
    │   ├── datasource.rs
    │   └── mod.rs
    ├── dto
    │   ├── mod.rs
    │   ├── token_request.rs
    │   └── token_response.rs
    ├── errors
    │   ├── api_error.rs
    │   ├── auth_error.rs
    │   └── mod.rs
    ├── handlers
    │   ├── macros.rs
    │   ├── mod.rs
    │   ├── student.rs
    │   ├── subject.rs
    │   └── user.rs
    ├── hash_helpers.rs
    ├── main.rs
    ├── middlewares
    │   ├── auth.rs
    │   └── mod.rs
    ├── models
    │   ├── mark.rs
    │   ├── mod.rs
    │   ├── role.rs
    │   ├── stage.rs
    │   ├── student.rs
    │   ├── subject.rs
    │   ├── user_role.rs
    │   └── user.rs
    ├── repository
    │   ├── common.rs
    │   ├── mod.rs
    │   ├── role.rs
    │   ├── stage.rs
    │   ├── student.rs
    │   ├── subject.rs
    │   └── user.rs
    ├── schema.rs
    └── tests
        ├── mod.rs
        └── user.rs


````   

# Build and run

1. Clone a repository

````   
git clone http://github.com/an-nguen/ediary-app
````

2. Go to directory
````
cd ./ediary-app
````
3. Build and run app
````
cargo check
cargo build --release
cargo run
````
