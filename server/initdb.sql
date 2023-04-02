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
