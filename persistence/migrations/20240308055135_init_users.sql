-- Add migration script here
CREATE TYPE permissions_type AS ENUM ('user', 'admin', 'super_admin');
CREATE TABLE IF NOT EXISTS users (
    id UUID primary key not null,
    username varchar(20),
    password varchar(255),
    access_token varchar(255),
    permissions permissions_type DEFAULT 'user'
);
