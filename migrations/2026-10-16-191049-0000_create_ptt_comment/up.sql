-- Your SQL goes here
CREATE TYPE comment_type_enum AS ENUM ('recommend', 'boo', 'comment', 'reply', 'forward');

CREATE TABLE ptt_comment (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    comment_id INT NOT NULL,
    the_type comment_type_enum,
    ip TEXT,
    host TEXT,
    the_date TEXT,
    dbcs TEXT,
    edit_time_us TIMESTAMPTZ,
    content_md5 TEXT,
    CONSTRAINT fk_comment
        FOREIGN KEY (comment_id)
        REFERENCES comment(id)
        ON DELETE CASCADE
);
