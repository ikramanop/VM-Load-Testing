create extension if not exists "uuid-ossp";

create table task
(
    id         serial  not null primary key,
    uuid       uuid    not null default uuid_generate_v4(),
    endpoints  text[]  not null,
    status     text    not null,
    iterations integer not null default 1000,
    meta       text    not null
);

create table queue
(
    id       serial  not null primary key,
    task_id  integer not null references task (id),
    endpoint text    not null,
    response float   null,
    status   integer not null default 0
);