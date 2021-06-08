/*
 There are 3 types of tweets that we distinguish:
 1. "normal" tweets = those posted by people we follow
 2. "rt_original" tweets = original tweets for any retweets done by the people we follow
 3. "helper" tweets = those that are quoted / replied to

 Depending on the type of tweet, we process them differently. In v2 api:
 1. "normal" tweets already have all the data they need to display correctly returned in a call to user_timeline
 2. "rt_original" tweets are missing 1)media and 2)helper tweets, which we need to fill separately
 3. "helper" tweets are also missing 1) and 2) above, but we only care to pull 1)
 */
-- CREATE TYPE tweet_class AS ENUM ('normal', 'rt_original', 'helper');

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
    quoted_tweet_id     TEXT,
    tweet_class         TEXT        NOT NULL,

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
    id          uuid        NOT NULL,
    PRIMARY KEY (id),
    created_at  timestamptz NOT NULL,

    -- core twitter stuff
    media_key   TEXT        NOT NULL UNIQUE,
    media_type  TEXT,
    display_url TEXT,

    -- relation to tweets
    tweet_id    uuid        NOT NULL,
    FOREIGN KEY (tweet_id)
        REFERENCES tweets (id)
);

CREATE INDEX tweet_created_at_index ON tweets (tweet_created_at);

CREATE INDEX popularity_count_index ON tweets (popularity_count);
CREATE INDEX popularity_count_special_index ON tweets (CAST(popularity_count || LEFT(tweet_id, 10) AS BIGINT));

CREATE INDEX like_count_index ON tweets (like_count);
CREATE INDEX like_count_special_index ON tweets (CAST(like_count || LEFT(tweet_id, 10) AS BIGINT));

CREATE INDEX quote_count_index ON tweets (quote_count);
CREATE INDEX quote_count_special_index ON tweets (CAST(quote_count || LEFT(tweet_id, 10) AS BIGINT));

CREATE INDEX reply_count_index ON tweets (reply_count);
CREATE INDEX reply_count_special_index ON tweets (CAST(reply_count || LEFT(tweet_id, 10) AS BIGINT));

CREATE INDEX retweet_count_index ON tweets (retweet_count);
CREATE INDEX retweet_count_special_index ON tweets (CAST(retweet_count || LEFT(tweet_id, 10) AS BIGINT));

CREATE INDEX total_retweet_count_index ON tweets (total_retweet_count);
CREATE INDEX total_retweet_count_special_index ON tweets (CAST(total_retweet_count || LEFT(tweet_id, 10) AS BIGINT));