CREATE TABLE users(
    user_id VARCHAR(64),
    email VARCHAR(64),
    username VARCHAR(32),
    first_name VARCHAR(32),
    last_name VARCHAR(32),
    age TINYINT,
    password_hash VARCHAR(64),
    profile_pic VARCHAR(255),
    is_email_verified BOOLEAN,
    UNIQUE (user_id),
    UNIQUE (email)
    );

CREATE TABLE otps{
    user_id VARCHAR(64),
    otp VARCHAR(4),
    FOREIGN KEY (user_id) REFERENCES users(user_id)
};
