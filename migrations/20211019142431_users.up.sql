CREATE TABLE users (
    id INT GENERATED ALWAYS AS IDENTITY,
    email VARCHAR UNIQUE NOT NULL,
    username VARCHAR NOT NULL,
    psswd VARCHAR NOT NULL,
    PRIMARY KEY (id)
);
CREATE INDEX users_email on users (email);
