-- Your SQL goes here
create table stage (
    id serial primary key,
    start_date date not null,
    end_date date not null,
    student_id integer references student (id),
    stage varchar(32) not null
)