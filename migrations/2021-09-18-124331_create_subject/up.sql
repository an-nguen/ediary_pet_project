-- Your SQL goes here
create table subject (
    id serial primary key,
    name varchar(256) not null unique,
    description varchar(4096)
);