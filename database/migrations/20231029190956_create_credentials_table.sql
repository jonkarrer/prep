CREATE TABLE credentials (
    credential_id CHAR(36) PRIMARY KEY,
    user_identity VARCHAR(255) NOT NULL,
    hashed_password VARCHAR(255) NOT NULL
)