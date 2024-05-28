-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
    id UUID primary key not null,
    username varchar(20),
    password varchar(255),
    access_token varchar(255)
);
