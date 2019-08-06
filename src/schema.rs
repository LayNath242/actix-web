table! {
    comments (id) {
        id -> Int4,
        description -> Varchar,
        user_id -> Int4,
        created_at -> Nullable<Timestamp>,
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

joinable!(comments -> users (user_id));
joinable!(users -> roles (role_id));

allow_tables_to_appear_in_same_query!(
    comments,
    roles,
    users,
);
