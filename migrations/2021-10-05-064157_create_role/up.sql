-- Your SQL goes here
create table role (
    id serial primary key,
    name varchar(128) unique not null ,
    description varchar(2048)
);

insert into role (name, description) values ('ADMIN', '');
insert into role (name, description) values ('USER', '');