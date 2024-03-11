-- Add migration script here
CREATE TABLE IF NOT EXISTS users (
    id UUID primary key not null,
    username varchar(20),
);

CREATE TABLE IF NOT EXISTS sessions (
    session_token varchar(255) primary key not null,
    user_id UUID not null
);
