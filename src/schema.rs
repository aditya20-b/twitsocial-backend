// @generated automatically by Diesel CLI.

diesel::table! {
    followers (user_id, follower_id) {
        user_id -> Int4,
        follower_id -> Int4,
    }
}

diesel::table! {
    likes (user_id, tweet_id) {
        user_id -> Int4,
        tweet_id -> Int4,
    }
}

diesel::table! {
    replies (reply_id) {
        reply_id -> Int4,
        tweet_id -> Nullable<Int4>,
        user_id -> Nullable<Int4>,
        reply_content -> Text,
        date_created -> Timestamptz,
        date_updated -> Timestamptz,
    }
}

diesel::table! {
    tweets (tweet_id) {
        tweet_id -> Int4,
        user_id -> Nullable<Int4>,
        content -> Text,
        date_created -> Timestamptz,
        date_updated -> Timestamptz,
    }
}

diesel::table! {
    useranalytics (user_id, date) {
        user_id -> Int4,
        date -> Timestamptz,
        time_spent_on_app -> Nullable<Int4>,
    }
}

diesel::table! {
    userauth (user_id) {
        user_id -> Int4,
        #[max_length = 32]
        username -> Varchar,
        #[max_length = 32]
        email -> Varchar,
        #[max_length = 100]
        password_hash -> Varchar,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Int4,
        #[max_length = 32]
        username -> Varchar,
        #[max_length = 50]
        handle -> Varchar,
        is_private -> Bool,
        bio -> Nullable<Text>,
        #[max_length = 255]
        profile_pic -> Nullable<Varchar>,
        #[max_length = 255]
        banner -> Nullable<Varchar>,
        date_created -> Timestamptz,
        date_updated -> Timestamptz,
    }
}

diesel::joinable!(likes -> tweets (tweet_id));
diesel::joinable!(likes -> users (user_id));
diesel::joinable!(replies -> tweets (tweet_id));
diesel::joinable!(replies -> userauth (user_id));
diesel::joinable!(tweets -> users (user_id));
diesel::joinable!(useranalytics -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    followers,
    likes,
    replies,
    tweets,
    useranalytics,
    userauth,
    users,
);
