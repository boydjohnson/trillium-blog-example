CREATE TABLE tags(
    id INT GENERATED ALWAYS AS IDENTITY,
    tag VARCHAR NOT NULL,
    PRIMARY KEY (id)
);
CREATE INDEX tags_tag on tags (tag);
