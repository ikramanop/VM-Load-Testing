create table if not exists provider
(
    id         serial  not null primary key,
    name       text    not null,
    user_limit integer not null default 0
);

create table if not exists affiliation
(
    id           serial  not null primary key,
    provider_id  integer not null references provider (id),
    company_name text    not null
);

create table if not exists "user"
(
    id             serial    not null primary key,
    affiliation_id integer   not null references affiliation (id),
    name           text      not null,
    surname        text      not null,
    birth_date     timestamp not null
);

create table if not exists accounting
(
    id           serial    not null primary key,
    user_id      integer   not null references "user" (id),
    amount       integer   not null,
    payment_type text      not null,
    tax_charged  integer   not null,
    date_paid    timestamp not null
)

