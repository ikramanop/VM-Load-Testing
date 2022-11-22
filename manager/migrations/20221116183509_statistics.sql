create table if not exists statistics
(
    id      serial  not null primary key,
    task_id integer not null references task (id),
    avg     float   not null,
    std     float   not null,
    "min"   float   not null,
    "max"   float   not null
)