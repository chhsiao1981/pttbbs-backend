-- Your SQL goes here
CREATE TABLE ptt_community (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    community_id INT NOT NULL,
    ptt_brdname TEXT,
    bclass TEXT,
    brdattr INT NOT NULL DEFAULT 0,
    vote_limit_logins INT NOT NULL DEFAULT 0,
    post_limit_logins INT NOT NULL DEFAULT 0,
    bvote INT NOT NULL DEFAULT 0,
    vote_end_time TIMESTAMPTZ,
    perm INT NOT NULL DEFAULT 0,
    perm_update_time TIMESTAMPTZ,
    gamble_end_time TIMESTAMPTZ,
    fastrecommend_pause INT,
    vote_limit_badpost INT NOT NULL DEFAULT 0,
    post_limit_badpost INT NOT NULL DEFAULT 0,
    bottom INT[] NOT NULL,
    CONSTRAINT fk_community
        FOREIGN KEY (community_id)
        REFERENCES community(id)
        ON DELETE CASCADE
);
