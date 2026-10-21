-- Your SQL goes here
CREATE TABLE ptt_post (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    post_id INT NOT NULL,
    ptt_id TEXT,
    class TEXT, -- classes defined by moderators.
    content TEXT, -- json-encoded Rune[][]. body is markdown or html.
    money INT DEFAULT 0 NOT NULL,
    ip TEXT,
    host TEXT,
    bbs TEXT,
    is_highlight BOOLEAN DEFAULT FALSE NOT NULL,
    full_title TEXT, -- for debugging.
    full_title_big5 BYTEA, -- if integrated with big5 system.
    content_big5 BYTEA, -- if integrated with big5 system.
    signature_big5 BYTEA, -- if integrated with big5 system.
    content_big5_md5 TEXT, -- for quick comparison to determine whether to update.
    signature_big5_md5 TEXT, -- for quick comparison to determine whether to update.
    CONSTRAINT fk_post
        FOREIGN KEY (post_id)
        REFERENCES post(id)
        ON DELETE CASCADE
);

-- cached comment info for a post.
CREATE TABLE ptt_post_comment (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    recommend INT DEFAULT 0 NOT NULL,
    n_comment INT DEFAULT 0 NOT NULL,
    post_id INT NOT NULL,
    CONSTRAINT fk_ptt_post_comment
        FOREIGN KEY (post_id)
        REFERENCES post(id)
        ON DELETE CASCADE
);