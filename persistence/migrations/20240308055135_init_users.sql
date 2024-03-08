-- Add migration script here
CREATE TABLE users (
    id UUID primary key not null,
    username varchar(20),
    password varchar(20)

);
