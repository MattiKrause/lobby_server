CREATE EXTENSION IF NOT EXISTS pgcrypto;
CREATE TABLE IF NOT EXISTS users (
    user_id uuid PRIMARY KEY ,
    user_name text NOT NULL,
    user_email text NOT NULL UNIQUE,
    user_password text NOT NULL
);
CREATE TABLE IF NOT EXISTS tokens (
    token uuid PRIMARY KEY,
    user_id uuid NOT NULL,
    creation_date timestamp NOT NULL,
    last_accessed timestamp NOT NULL,
    FOREIGN KEY (user_id)
        REFERENCES users (user_id)
)