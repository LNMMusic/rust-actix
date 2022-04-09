DROP SCHEMA IF EXISTS testing CASCADE;
CREATE SCHEMA testing;

CREATE TABLE IF NOT EXISTS testing.users (
    id          BIGSERIAL PRIMARY KEY,
    username    VARCHAR(200) UNIQUE NOT NULL,
    password    VARCHAR(200) NOT NULL,
    
    fullname    VARCHAR(200),
    email       VARCHAR(200),
    UNIQUE (username)
);