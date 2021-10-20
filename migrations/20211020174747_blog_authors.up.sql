CREATE TABLE blog_authors (
    id INT GENERATED ALWAYS AS IDENTITY,
    blog_id INT NOT NULL,
    user_id INT NOT NULL,
    accepted BOOLEAN DEFAULT FALSE,
    FOREIGN KEY (blog_id) REFERENCES blogs (id),
    FOREIGN KEY (user_id) REFERENCES users (id),
    PRIMARY KEY (id)    
);
