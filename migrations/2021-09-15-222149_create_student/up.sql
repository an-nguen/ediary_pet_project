-- Your SQL goes here
create table student (
    id serial primary key,
    first_name varchar(256) not null,
    last_name varchar(256) not null,
    birth_date date
)