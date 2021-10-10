-- Your SQL goes here
create table acl (
    id serial primary key,
    resource_path varchar(256) unique not null,
    group_id_owner int not null,
    group_read_access bool not null,
    group_write_access bool not null,
    all_read_access bool not null,
    all_write_access bool not null

)