CREATE TABLE tags_posts(
    id INT GENERATED ALWAYS AS IDENTITY,
    tag_id INT NOT NULL,
    post_id INT NOT NULL,
    PRIMARY KEY (id),
    FOREIGN KEY (tag_id) REFERENCES tags(id),
    FOREIGN KEY (post_id) REFERENCES posts(id)
);
CREATE INDEX tags_posts_tags_posts ON tags_posts(tag_id, post_id);
