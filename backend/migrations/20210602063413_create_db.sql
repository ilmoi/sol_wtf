CREATE TABLE users
(
    -- basics
    id              uuid        NOT NULL,
    PRIMARY KEY (id),
    created_at      timestamptz NOT NULL,

    -- core twitter stuff
    twitter_user_id TEXT        NOT NULL UNIQUE,
    twitter_name    TEXT        NOT NULL,
    twitter_handle  TEXT        NOT NULL,
    profile_url     TEXT        NOT NULL,
    profile_image   TEXT, -- can be null? didn't dig into

    -- v2 metrics
    followers_count BIGINT DEFAULT 0,
    following_count BIGINT DEFAULT 0,
    listed_count    BIGINT DEFAULT 0,
    tweet_count     BIGINT DEFAULT 0,

    -- v1 backup
    entire_user     json

);

CREATE TABLE tweets
(
    -- basics
    id                  uuid        NOT NULL,
    PRIMARY KEY (id),
    created_at          timestamptz NOT NULL,

    -- core twitter stuff
    tweet_id            TEXT        NOT NULL UNIQUE,
    tweet_created_at    timestamptz NOT NULL,
    tweet_text          TEXT        NOT NULL,
    tweet_url           TEXT        NOT NULL,

    -- referenced tweets
    replied_to_tweet_id TEXT,
    is_reply            BOOL   DEFAULT false,
    quoted_tweet_id     TEXT,
    is_quote            BOOL   DEFAULT false,
    retweeted_tweet_id  TEXT,
    is_retweet          BOOL   DEFAULT false,

    -- v2 metrics
    like_count          BIGINT DEFAULT 0,
    quote_count         BIGINT DEFAULT 0,
    reply_count         BIGINT DEFAULT 0,
    retweet_count       BIGINT DEFAULT 0,
    total_retweet_count BIGINT DEFAULT 0,
    popularity_count    BIGINT DEFAULT 0,

    -- v1 backup
    entire_tweet        json,

    -- relation to users
    user_id             uuid        NOT NULL,
    FOREIGN KEY (user_id)
        REFERENCES users (id)

);

CREATE TABLE media
(
    -- basics
    id                uuid        NOT NULL,
    PRIMARY KEY (id),
    created_at        timestamptz NOT NULL,

    -- core twitter stuff
    media_key         TEXT        NOT NULL UNIQUE,
    media_type        TEXT        NOT NULL,
    preview_image_url TEXT,

    -- relation to tweets
    tweet_id          uuid        NOT NULL,
    FOREIGN KEY (tweet_id)
        REFERENCES tweets (id)
)