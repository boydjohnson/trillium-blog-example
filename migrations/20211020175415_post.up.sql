CREATE TABLE posts (
    id INT GENERATED ALWAYS AS IDENTITY,
    title VARCHAR NOT NULL,
    body VARCHAR NOT NULL,
    blog_id INT NOT NULL,
    FOREIGN KEY (blog_id) REFERENCES blogs(id),
    PRIMARY KEY (id)
);
