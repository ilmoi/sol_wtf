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
    profile_image   TEXT, -- can be null? didn't dig into
    profile_url     TEXT        NOT NULL,
    entire_user     json        NOT NULL
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

    -- metrics
    reply_count         INT  DEFAULT 0,
    like_count          INT  DEFAULT 0,
    quote_count         INT  DEFAULT 0,
    retweet_count       INT  DEFAULT 0,
    total_retweet_count INT  DEFAULT 0,
    popularity_count    INT  DEFAULT 0,

    -- display
    parent_tweet_id     TEXT,
    is_parent           BOOL DEFAULT false,
--     quoted_tweet_id     TEXT, <currently pulling from json
--     is_quote            BOOL DEFAULT false,
--     media-related stuff < currently pulling from json

    -- backup
    entire_tweet        json        NOT NULL,

    -- relation to users
    user_id             uuid        NOT NULL,
    FOREIGN KEY (user_id)
        REFERENCES users (id)

);