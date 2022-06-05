CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL UNIQUE,
    email VARCHAR NOT NULL UNIQUE,
    password VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL,
    last_login TIMESTAMP NOT NULL
);

CREATE TABLE spotify_tokens (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL UNIQUE,
    access_token VARCHAR NOT NULL,
    expires_in_seconds BIGINT NOT NULL,
    expires_at TIMESTAMP,
    refresh_token VARCHAR,
    scopes TEXT[] NOT NULL
);