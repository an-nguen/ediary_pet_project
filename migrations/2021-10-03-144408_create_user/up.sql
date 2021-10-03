-- Your SQL goes here
create table usr (
    id serial primary key,
    username varchar(256) unique not null,
    password_hash varchar(1024) not null,
    password_salt varchar(256) not null,
    email varchar(512) not null,
    birthday date not null,
    active bool not null default false,
    activation_token varchar(128)
)