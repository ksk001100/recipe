-- Add up migration script here
CREATE TABLE users (
    id BIGINT NOT NULL PRIMARY KEY AUTO_INCREMENT,
    name varchar(255) NOT NULL,
    email varchar(255) NOT NULL UNIQUE,
    created_at DATETIME NOT NULL default CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL default CURRENT_TIMESTAMP
)
