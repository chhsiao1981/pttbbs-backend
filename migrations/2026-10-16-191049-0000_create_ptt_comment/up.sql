-- Your SQL goes here
CREATE TYPE comment_type_enum AS ENUM ('recommend', 'boo', 'comment', 'reply', 'forward');

CREATE TABLE ptt_comment (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    comment_id INT NOT NULL,
    ptt_id TEXT,
    the_type comment_type_enum,
    ip TEXT,
    host TEXT,
    the_date TEXT,
    dbcs BYTEA,
    edit_time TIMESTAMPTZ, -- for reply.
    content_md5 TEXT,
    CONSTRAINT fk_comment
        FOREIGN KEY (comment_id)
        REFERENCES comment(id)
        ON DELETE CASCADE
);
