# school e-diary application (pet project)
****
**Backend** - REST API server written in Rust using **Rocket** framework + **Diesel** ORM.  
**Frontend** - ?
****
# Structure
****

    .
    ├── Cargo.lock
    ├── Cargo.toml
    ├── diesel.toml
    ├── frontend
    ├── migrations                  // Diesel migrations
    ├── README.md
    ├── Rocket.toml
    └── src
        ├── main.rs
        ├── models                      // Data models
        │   ├── error.rs
        │   ├── mark.rs
        │   ├── mod.rs
        │   ├── role.rs
        │   ├── stage.rs
        │   ├── student.rs
        │   ├── subject.rs
        │   ├── token_request.rs        // for acquire access token
        │   ├── token_response.rs       //
        │   └── user.rs
        ├── password_hash.rs                  // definition of Rocket.rs 'guard' which allow to signing and verifying passwords
        ├── repository
        │   ├── common.rs
        │   ├── mod.rs
        │   ├── student.rs
        │   ├── subject.rs
        │   └── user.rs
        ├── routes
        │   ├── mod.rs
        │   └── student.rs
        ├── schema.rs
        └── token.rs                          // access token 'guard' 


****
# Build and run
****
1. Clone a repository


    git clone http://github.com/an-nguen/ediary-app


2. Go to directory

    
    cd ./ediary-app

3. Build and run app


    cargo check
    cargo build --release
    cargo run