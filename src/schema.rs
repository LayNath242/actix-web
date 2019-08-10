table! {
    categories (id) {
        id -> Int4,
        title -> Varchar,
        user_id -> Int4,
        created_at -> Nullable<Timestamp>,
    }
}

table! {
    comments (id) {
        id -> Int4,
        description -> Varchar,
        user_id -> Int4,
        created_at -> Nullable<Timestamp>,
    }
}

table! {
    courses (id) {
        id -> Int4,
        title -> Varchar,
        thumbnail -> Nullable<Varchar>,
        video_url -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        cate_id -> Int4,
        price -> Float8,
        created_at -> Date,
    }
}

table! {
    roles (id) {
        id -> Int4,
        title -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}

table! {
    user_courses (id) {
        id -> Int4,
        user_id -> Int4,
        course_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        fullname -> Varchar,
        email -> Varchar,
        password -> Varchar,
        avatar -> Nullable<Varchar>,
        biography -> Nullable<Varchar>,
        created_at -> Timestamp,
        role_id -> Int4,
    }
}

joinable!(categories -> users (user_id));
joinable!(comments -> users (user_id));
joinable!(courses -> categories (cate_id));
joinable!(user_courses -> courses (course_id));
joinable!(user_courses -> users (user_id));
joinable!(users -> roles (role_id));

allow_tables_to_appear_in_same_query!(
    categories,
    comments,
    courses,
    roles,
    user_courses,
    users,
);
