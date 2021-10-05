-- Add up migration script here
CREATE TABLE users (
    id BIGINT NOT NULL PRIMARY KEY AUTO_INCREMENT,
    name varchar(255) NOT NULL,
    email varchar(255) NOT NULL,
    password_digest varchar(255) NOT NULL,
    created_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL default CURRENT_TIMESTAMP
)
