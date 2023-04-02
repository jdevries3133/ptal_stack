create extension if not exists "uuid-ossp";

-- Ensure the database and schema names here match the databaes and schema
-- name in the `.env` file.
create database ptal_stack_sql;
create schema ptal_stack_sql;

\c ptal_stack_sql;

create table users(
    id serial primary key,
    username varchar(255) not null,
    email varchar(255) not null
);

create table password(
    id serial primary key,
    salt varchar(255) not null,
    digest varchar(255) not null,
    created timestamp with time zone default now() not null,

    user_id int references users(id) on delete cascade unique not null
);

create table anonymous_req_count(
    id serial primary key,
    time timestamp with time zone default now() not null
);

create table dogs(
    id serial primary key,
    day date unique default now() not null,
    href varchar(255) not null
);
