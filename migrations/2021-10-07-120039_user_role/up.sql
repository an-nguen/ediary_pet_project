-- Your SQL goes here
create table user_role (
    id serial primary key,
    user_id int references usr(id) on update cascade on delete cascade not null,
    role_id int references role(id) on update cascade on delete cascade not null
)