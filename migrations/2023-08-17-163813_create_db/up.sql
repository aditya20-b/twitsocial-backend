-- Your SQL goes here
-- Table: userauth
CREATE TABLE IF NOT EXISTS USERAUTH (
    user_id SERIAL,
    username VARCHAR(32) NOT NULL UNIQUE,
    email VARCHAR(32) NOT NULL UNIQUE,
    password_hash VARCHAR(100) NOT NULL,
    PRIMARY KEY (user_id)
);

-- Table: users
CREATE TABLE IF NOT EXISTS USERS (
    user_id INT,
    username VARCHAR(32) NOT NULL UNIQUE,
    handle VARCHAR(50) NOT NULL,
    is_private BOOLEAN DEFAULT FALSE,
    bio TEXT,
    profile_pic VARCHAR(255),
    banner VARCHAR(255),
    date_created TIMESTAMP DEFAULT NOW(),
    PRIMARY KEY (user_id),
    FOREIGN KEY (user_id) REFERENCES USERAUTH(user_id),
    FOREIGN KEY (username) REFERENCES USERAUTH(username)
);

-- Table: tweets
CREATE TABLE IF NOT EXISTS TWEETS (
    tweet_id SERIAL,
    user_id INT,
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    PRIMARY KEY (tweet_id),
    FOREIGN KEY (user_id) REFERENCES USERS(user_id)
);

-- Table: likes
CREATE TABLE IF NOT EXISTS LIKES (
    user_id INTEGER,
    tweet_id INTEGER,
    PRIMARY KEY (user_id, tweet_id),
    FOREIGN KEY (user_id) REFERENCES USERS(user_id),
    FOREIGN KEY (tweet_id) REFERENCES TWEETS(tweet_id)
);


-- Table: replies
CREATE TABLE IF NOT EXISTS REPLIES (
    reply_id SERIAL,
    tweet_id INTEGER,
    user_id INTEGER,
    reply_content TEXT NOT NULL,
    date_created TIMESTAMPTZ DEFAULT NOW(),
    PRIMARY KEY (reply_id),
    FOREIGN KEY (tweet_id) REFERENCES TWEETS(tweet_id),
    FOREIGN KEY (user_id) REFERENCES USERAUTH(user_id)
);


-- Table: followers
CREATE TABLE IF NOT EXISTS FOLLOWERS (
    user_id INTEGER,
    follower_id INTEGER,
    PRIMARY KEY (user_id, follower_id),
    FOREIGN KEY (user_id) REFERENCES USERS(user_id),
    FOREIGN KEY (follower_id) REFERENCES USERS(user_id)
);

-- Table: useranalytics
CREATE TABLE IF NOT EXISTS USERANALYTICS (
    user_id INTEGER,
    date TIMESTAMPTZ DEFAULT NOW(),
    time_spent_on_app INTEGER,
    PRIMARY KEY (user_id, date),
    FOREIGN KEY (user_id) REFERENCES USERS(user_id)
);
