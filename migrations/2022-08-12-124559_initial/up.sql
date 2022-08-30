-- Your SQL goes here
create table if not exists frames (
    id varchar(255) primary key not null,
    start datetime not null,
    end datetime DEFAULT null,
    last_update datetime not null,
    project varchar(255) not null,
    tags varchar(2048) not null,
    deleted boolean not null
);
