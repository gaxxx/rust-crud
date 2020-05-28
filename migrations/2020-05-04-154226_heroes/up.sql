-- Your SQL goes here

create table users (
id serial primary key,
name varchar not null  default '',
identity varchar not null default '',
hometown varchar not null default '',
age int not null default 0
);