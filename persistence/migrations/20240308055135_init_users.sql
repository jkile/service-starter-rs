-- Add migration script here
CREATE TYPE permissions_type AS ENUM ('user', 'admin', 'super_admin');
CREATE TABLE IF NOT EXISTS users (
    id UUID primary key NOT NULL,
    username varchar(20) NOT NULL,
    password varchar(255) NOT NULL,
    access_token varchar(255),
    permissions_type permissions_type DEFAULT 'user' NOT NULL
);
