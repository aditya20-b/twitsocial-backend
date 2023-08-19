DROP TABLE USERS;

CREATE TABLE IF NOT EXISTS USERS (
    userid INTEGER,
    username CHAR(32) NOT NULL UNIQUE,
    handle VARCHAR(50) NOT NULL,
    is_private BOOLEAN DEFAULT FALSE,
    bio TEXT,
    profile_pic VARCHAR(255),
    banner VARCHAR(255),
    date_created TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (userid) REFERENCES USERAUTH(UserID),
    FOREIGN KEY (username) REFERENCES USERAUTH(UserName)
);

INSERT INTO USERS (userid, username, handle, is_private, bio, profile_pic, banner)
VALUES
    (1, 'john_doe', 'JohnDoe', FALSE, 'I love coding and hiking.', 'profile_pic_1.jpg', 'banner_1.jpg'),
    (2, 'jane_smith', 'JaneSmith', TRUE, 'Software engineer by day, artist by night.', 'profile_pic_2.jpg', 'banner_2.jpg'),
    (3, 'sam_jones', 'SamJones', FALSE, 'Just a regular guy.', 'profile_pic_3.jpg', 'banner_3.jpg'),
    (4, 'emily_wilson', 'EmilyWilson', FALSE, 'Coffee addict ☕️', 'profile_pic_4.jpg', 'banner_4.jpg'),
    (5, 'michael_adams', 'MikeAdams', TRUE, 'Gamer and tech enthusiast.', 'profile_pic_5.jpg', 'banner_5.jpg'),
    (6, 'sarah_brown', 'SarahBrown', FALSE, 'I love to travel.', 'profile_pic_6.jpg', 'banner_6.jpg'),
    (7, 'david_miller', 'DavidMiller', FALSE, 'I love to travel.', 'profile_pic_7.jpg', 'banner_7.jpg'),
    (8, 'laura_davis', 'LauraDavis', FALSE, 'I love to travel.', 'profile_pic_8.jpg', 'banner_8.jpg'),
    (9, 'james_wilson', 'JamesWilson', FALSE, 'I love to travel.', 'profile_pic_9.jpg', 'banner_9.jpg'),
    (10, 'mary_jones', 'MaryJones', FALSE, 'I love to travel.', 'profile_pic_10.jpg', 'banner_10.jpg'),
    (11, 'robert_taylor', 'RobertTaylor', FALSE, 'I love to travel.', 'profile_pic_11.jpg', 'banner_11.jpg'),
    (12, 'linda_anderson', 'LindaAnderson', FALSE, 'I love to travel.', 'profile_pic_12.jpg', 'banner_12.jpg'),
    (13, 'william_thomas', 'WilliamThomas', FALSE, 'I love to travel.', 'profile_pic_13.jpg', 'banner_13.jpg'),
    (14, 'elizabeth_wilson', 'ElizabethWilson', FALSE, 'I love to travel.', 'profile_pic_14.jpg', 'banner_14.jpg'),
    (15, 'richard_johnson', 'RichardJohnson', FALSE, 'I love to travel.', 'profile_pic_15.jpg', 'banner_15.jpg'),
    (16, 'barbara_martin', 'BarbaraMartin', FALSE, 'I love to travel.', 'profile_pic_16.jpg', 'banner_16.jpg'),
    (17, 'joseph_lee', 'JosephLee', FALSE, 'I love to travel.', 'profile_pic_17.jpg', 'banner_17.jpg'),
    (18, 'susan_clark', 'SusanClark', FALSE, 'I love to travel.', 'profile_pic_18.jpg', 'banner_18.jpg'),
    (19, 'thomas_rodriguez', 'ThomasRodriguez', FALSE, 'I love to travel.', 'profile_pic_19.jpg', 'banner_19.jpg'),
    (20, 'margaret_lewis', 'MargaretLewis', FALSE, 'I love to travel.', 'profile_pic_20.jpg', 'banner_20.jpg');


CREATE TABLE IF NOT EXISTS USERAUTH (
    UserID INTEGER,
    UserName CHAR(32) NOT NULL UNIQUE,
    Email CHAR(32) UNIQUE,
    PasswordHash VARCHAR(50) NOT NULL,
    PRIMARY KEY (UserID),
);

CREATE TABLE IF NOT EXISTS TWEETS (
    TweetID INTEGER PRIMARY KEY,
    UserID INTEGER,
    TweetContent TEXT NOT NULL,
    DateCreated TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (UserID) REFERENCES USERS(UserID)
);

CREATE TABLE IF NOT EXISTS USERANALYTICS (
    UserID INTEGER,
    Date TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    TimeSpentOnApp INTEGER,
    FOREIGN KEY (UserID) REFERENCES USERS(UserID)
);

CREATE TABLE IF NOT EXISTS LIKES (
    UserID INTEGER,
    TweetID INTEGER,
    PRIMARY KEY (UserID, TweetID),
    FOREIGN KEY (UserID) REFERENCES USERS(UserID),
    FOREIGN KEY (TweetID) REFERENCES TWEETS(TweetID)
);

CREATE TABLE IF NOT EXISTS REPLIES (
    ReplyID INTEGER PRIMARY KEY,
    TweetID INTEGER,
    UserID INTEGER,
    ReplyContent TEXT NOT NULL,
    DateCreated TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (TweetID) REFERENCES TWEETS(TweetID),
    FOREIGN KEY (UserID) REFERENCES USERS(UserID)
);

CREATE TABLE IF NOT EXISTS FOLLOWERS (
    UserID INTEGER,
    FollowerID INTEGER,
    PRIMARY KEY (UserID, FollowerID),
    FOREIGN KEY (UserID) REFERENCES USERS(UserID),
    FOREIGN KEY (FollowerID) REFERENCES USERS(UserID)
);

CREATE VIEW OR REPLACE PUBLICTWEETS AS
    SELECT UserID, TweetContent, DateCreated FROM TWEETS
    WHERE UserID NOT IN (SELECT UserID FROM USERS WHERE IsPrivate = 1);
                