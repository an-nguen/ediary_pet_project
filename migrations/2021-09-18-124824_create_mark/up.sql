-- Your SQL goes here
create table mark (
    id serial primary key,
    student_id integer not null,
    subject_id integer not null,
    title varchar(512),
    created_at date not null,
    updated_at date not null,
    mark varchar(32),
    author varchar(512) not null
)